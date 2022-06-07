# Installation

`rust_unciv_server` is meant to be run behind a reverse proxy.

This directory contains hardened systemd unit files as well as an example nginx
server block to securely run `rust_unciv_server`. To use them, copy the
`.service` and `.socket` unit files to a suitable path, for instance
`/etc/systemd/system`. Then copy the `.conf` to a path that is accessible to
your nginx installation, make relevant modifications to it such as the TLS
certificate paths and `include` it in your main `nginx.conf`. Finally, enable
`rust_unciv_server.socket` and reload your nginx configuration.

This is just an example configuration, any other desirable setup is left as an
exercise to the reader.
