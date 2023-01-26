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

use clap::{ArgAction, Parser};
use std::path::PathBuf;

/// Simple Unciv multiplayer server
#[derive(Parser)]
#[command(author, version, about, long_about = None, disable_help_flag = true)]
pub struct Args {
    /// Port to listen on
    #[arg(short, long)]
    pub port: Option<u16>,

    /// Hostname or socket to listen on
    #[arg(short, long)]
    pub host: Option<String>,

    /// Save file directory to use
    #[arg(short, long = "dir")]
    pub directory: Option<PathBuf>,

    /// Print help information
    #[arg(long, global = true, action = ArgAction::Help)]
    pub help: Option<bool>,
}
