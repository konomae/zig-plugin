# Zig plugin

[Zig](https://ziglang.org/) WASM plugin for [proto](https://github.com/moonrepo/proto).

> [!IMPORTANT]
> Zig is available in proto's official built-in toolchain as of [proto v0.62](https://moonrepo.dev/blog/proto-v0.62). We recommend using the built-in toolchain instead of this plugin.

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
cargo build --target wasm32-wasip1
```

Test the plugin by running `proto` commands.

```shell
proto install zig-test
proto versions zig-test
```
