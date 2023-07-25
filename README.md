# pebble
Fun mini "game engine" side project using Rust, ggez and lua

This project is (at least for now) only for learning purpose only.

# How to run

To run pebble, you need to open terminal and run

```bash
cargo run
```

However you can specify in what mode pebble need to be run

Pebble mode are : 

- release
- debug

In release mode. Only info, warn and error log will be available.

In debug mode. All the log (including pebble app) will be log.

To specify a mode, you need to specify it before running cargo run. Like this

```bash
PEBBLE_MODE=debug|release cargo run
```

In the future, config file will be available but that's not the focus for now.

# Changelog
[changelog](./CHANGELOG.md)