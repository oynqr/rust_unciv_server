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

mod app;
mod cmd;

use anyhow::{bail, Error};
use app::default_handler;
use async_fs::{remove_file, File};
use clap::Parser;
use cmd::Args;
use futures_lite::future;
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

fn pathbuf_to_static_path(buf: PathBuf) -> &'static Path {
    Box::leak(buf.into_boxed_path())
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

    trillium.run(default_handler(working_directory));
    Ok(())
}
