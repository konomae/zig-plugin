# Zig plugin

[Zig](https://ziglang.org/) WASM plugin for [proto](https://github.com/moonrepo/proto).

## Installation

Add the following to `.prototools`.

```toml
[plugins]
zig = "github://konomae/zig-plugin"
```

## Configuration

Zig plugin does not support configuration.

## Hooks

Zig plugin does not support hooks.

## Contributing

Build the plugin:

```shell
cargo build --target wasm32-wasi
```

Test the plugin by running `proto` commands.

```shell
proto install zig-test
proto list-remote zig-test
```
