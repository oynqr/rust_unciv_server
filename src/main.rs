// Copyright (C) 2022 Philipp David <pd@3b.pm>

// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU Affero General Public License as published by the Free
// Software Foundation, version 3.

// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
// details.

// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>

#![forbid(unsafe_code)]

use anyhow::{bail, Error};
use async_fs::{remove_file, rename, File};
use clap::Parser;
use futures_lite::{future, io, stream::repeat_with, StreamExt};
use log::Level;
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};
use trillium::{conn_try, conn_unwrap, Conn};
use trillium_forwarding::Forwarding;
use trillium_logger::{apache_common, ColorMode, Logger, Target};
use trillium_router::{Router, RouterConnExt};

fn pathbuf_to_static_path(buf: PathBuf) -> &'static Path {
    Box::leak(buf.into_boxed_path())
}

/// Simple Unciv multiplayer server
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Port to listen on
    #[clap(short, long)]
    port: Option<u16>,

    /// Hostname or socket to listen on
    #[clap(short, long)]
    host: Option<String>,

    /// Save file directory to use
    #[clap(short, long = "dir")]
    directory: Option<PathBuf>,
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let args = Args::parse();

    let working_directory = if let Some(str) = args.directory {
        str
    } else {
        current_dir()?
    };
    if !working_directory.exists() {
        bail!("Directory does not exist: {}", working_directory.display());
    };
    if !working_directory.is_dir() {
        bail!("Path is not a directory: {}", working_directory.display());
    }

    let working_directory =
        pathbuf_to_static_path(working_directory.canonicalize()?);
    let testfile_path = working_directory.join("test");
    future::block_on(async move {
        File::create(&testfile_path).await?;
        remove_file(&testfile_path).await
    })?;

    let mut trillium = trillium_smol::config();

    if let Some(port) = args.port {
        trillium = trillium.with_port(port);
    }

    if let Some(host) = args.host {
        trillium = trillium.with_host(&host);
    }

    trillium.run((
        Logger::new()
            .with_formatter(apache_common("-", "-"))
            .with_color_mode(ColorMode::Off)
            .with_target(Target::Logger(Level::Info)),
        Forwarding::trust_always(),
        Router::new()
            .get("/isalive", |conn: Conn| async { conn.ok("true") })
            .get("/files/*", trillium_static::files(working_directory))
            .put("/files/:file", |mut conn: Conn| async {
                let path = conn_unwrap!(conn.param("file"), conn);
                let mut tmp_path = None;
                while tmp_path.is_none() {
                    let rand: String = repeat_with(fastrand::alphanumeric)
                        .take(16)
                        .collect()
                        .await;
                    let path = working_directory.join([path, &rand].concat());
                    if !path.exists() {
                        tmp_path = Some(path)
                    }
                }
                let tmp_path = conn_unwrap!(tmp_path, conn);
                let path = working_directory.join(path);
                let file = conn_try!(
                    File::create(&tmp_path).await,
                    conn.with_body("Failed to create file.")
                );
                conn_try!(
                    io::copy(conn.request_body().await, file).await,
                    conn.with_body("Failed to write request body.")
                );
                conn_try!(rename(&tmp_path, &path).await, {
                    conn_try!(
                        remove_file(&tmp_path).await,
                        conn.with_body(
                            "Failed to rename file. Failed to remove \
                             temporary file."
                        )
                    );
                    conn.with_body("Failed to rename file.")
                });
                conn.with_status(200)
            })
            .delete("/files/:file", |conn: Conn| async {
                let path = conn_unwrap!(conn.param("file"), conn);
                let path = working_directory.join(path);
                if !path.is_file() {
                    return conn;
                }
                conn_try!(remove_file(&path).await, conn);
                conn.with_status(200)
            }),
    ));
    Ok(())
}
