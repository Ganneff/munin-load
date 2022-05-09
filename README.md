# load

A really simple munin plugin to collect load statistics.

A drop-in replacement for the shell version. Mostly as a test-bed for
[my munin-plugin library for
Rust](https://github.com/Ganneff/munin-plugin), though it is fully
functional.

## How
Parsing procfs.

## Usage
Compile (or load a released binary, if one is there) and put the
binary somewhere. Then link it into the munin plugins dir.

## Installation
1. You need a rust environment setup. If you have none yet, please
   follow the instructions at either [the Rust Lang Get Started
   Page](https://www.rust-lang.org/learn/get-started) or
   [rustup.rs](https://rustup.rs/)
1. Download or clone this repo
1. Inside your copy, run `cargo build --release --target
   x86_64-unknown-linux-musl` (you can skip the --target[..] part, if
   your target system has a recent enough libc).
1. Copy `target/x86_64-unknown-linux-musl/release/load` (or
   `target/release/load` if you skipped target) to a useful
   place on your system and link it into `/etc/munin/plugins/` (or
   place it directly into that directory).
1. Restart munin-node (on Debian: `systemctl munin-node restart`)

## Musl
The build using musl creates fully static binaries. "Normal" rust link
against libc, and that may carry symbols that aren't available
everywhere (older versions). If you do not have that requirement, not
using musl will be fine.
