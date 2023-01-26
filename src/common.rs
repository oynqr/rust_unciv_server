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

use std::path::{Path, PathBuf};

use futures_lite::{stream::repeat_with, StreamExt};

pub async fn get_unique_path(
    path: &str,
    working_directory: &'static Path,
) -> PathBuf {
    loop {
        let rand: String =
            repeat_with(fastrand::alphanumeric).take(16).collect().await;
        let path = working_directory.join([path, &rand].concat());
        if !path.exists() {
            return path;
        }
    }
}
