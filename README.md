# rust_unciv_server

`rust_unciv_server` is a simple multiplayer server for
[Unciv](https://github.com/yairm210/Unciv). The aim is to create an
implementation that is faster, smaller and safer than the original one.

## Running

You can obtain binaries from the
[releases](https://github.com/oynqr/rust_unciv_server/releases). These binaries
are statically linked and have no additional dependencies. If there is no
applicable build for your platform or architecture, or you simply wish to build
the server yourself, refer to [Building](#building).

## Configuration

`rust_unciv_server` offers multiple configuration options, some of which can be
specified by passing environment variables, command line flags, or both. When
there are both an environment variable and a command line flag for the same
option, the command line flag will take precedence.

Here is an overview of available options:

Environment variable | Command line flag | Description
--- | --- | ---
RUST_LOG | - | Log level used for the entire application
HOST | -h | Address or unix socket to listen on, defaults to localhost
PORT | -p | Port to listen on, defaults to 8080
| - | -d | Save file directory to use, defaults to current working directory

## Installation

See [INSTALL.md](install/).

## Building

You will need a stable or nightly Rust installation to build
`rust_unciv_server`. Once you have one, enter the source directory and run
`cargo build` for a debug build or `cargo build --release` for an optimized
release build. If you wish to cross compile or enable advanced build options,
refer to the relevant sections in the
[rustup](https://rust-lang.github.io/rustup/) or the
[Cargo](https://doc.rust-lang.org/cargo/) book.
