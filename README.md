# About cgen

cgen is a C/C++ header generator built in Rust. It supports almost all of C, even C23, due to it being built around Libclang.

# Installation

The recommended installation is with Nix. Add it as an input to your devshells, or install it imperatively with

```sh
nix profile add github:corbinwunderlich/cgen
```

# Configuration

cgen can be configured through a cgen configuration file, located in the current working directory. It can be YAML, JSON, TOML, or any of the config types that [config-rs](https://crates.io/crates/config) supports.

> [!TIP]
> It is recommended to add the following (depending on your file format) to your config for LSP documentation:
>
> JSON:
>
> ```json
> {
>     "$schema": "https://raw.githubusercontent.com/corbinwunderlich/cgen/main/config.schema.json"
> }
> ```
>
> YAML:
>
> ```yaml
> # yaml-language-server: $schema=https://raw.githubusercontent.com/corbinwunderlich/cgen/main/config.schema.json
> ```
>
> TOML:
>
> ```toml
> #:schema https://raw.githubusercontent.com/corbinwunderlich/cgen/main/config.schema.json
> ```

# Planned

- Bindings generation
- Rust frontend
