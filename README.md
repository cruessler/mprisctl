# MPRISctl

MPRISctl is a small command line utility for controlling MPRIS-enabled media
players. It is written in Rust and aims to be compatible with
[acrisci/playerctl](https://github.com/acrisci/playerctl/).

## Installation

Provided you have `cargo` installed, installation is as easy as

```
cargo install https://github.com/cruessler/mprisctl
```

This will download the source code and compile the binary which can then be
found in `~/.cargo/bin`. If that’s in your `$PATH`, you can type `mprisctl
--help` to get an overview of the available commands.
