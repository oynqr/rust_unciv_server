// Copyright (C) 2022-2023 Philipp David <pd@3b.pm>

// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU Affero General Public License as published by the Free
// Software Foundation, version 3.

// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
// details.

// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>

use async_fs::{remove_file, rename, File};
use std::path::Path;

use futures_lite::io;
use log::Level;
use trillium::{conn_try, conn_unwrap, Conn, Handler, State};
use trillium_forwarding::Forwarding;
use trillium_logger::{apache_common, ColorMode, Logger, Target};
use trillium_router::{Router, RouterConnExt};

use crate::common::get_unique_path;

async fn upload_handler(mut conn: Conn) -> Conn {
    let path = conn_unwrap!(conn.param("file"), conn);
    let tmp_path =
        get_unique_path(path, conn.state::<&'static Path>().unwrap()).await;
    let path = conn.state::<&'static Path>().unwrap().join(path);
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
                "Failed to rename file. Failed to remove temporary file."
            )
        );
        conn.with_body("Failed to rename file.")
    });
    conn.with_status(200)
}

async fn delete_handler(conn: Conn) -> Conn {
    let path = conn_unwrap!(conn.param("file"), conn);
    let path = conn.state::<&'static Path>().unwrap().join(path);
    if !path.is_file() {
        return conn;
    }
    conn_try!(remove_file(&path).await, conn);
    conn.with_status(200)
}

pub fn default_handler(working_directory: &'static Path) -> impl Handler {
    (
        State::new(working_directory),
        Logger::new()
            .with_formatter(apache_common("-", "-"))
            .with_color_mode(ColorMode::Off)
            .with_target(Target::Logger(Level::Info)),
        Forwarding::trust_always(),
        Router::new()
            .get("/isalive", "true")
            .get("/files/*", trillium_static::files(working_directory))
            .put("/files/:file", upload_handler)
            .delete("/files/:file", delete_handler),
    )
}
