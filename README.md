# About cgen

cgen is a C/C++ header generator built in Rust. It supports almost all of C, even C23, due to it being built around Libclang.

# Installation

The recommended installation is with Nix. You can...

Add it as an input to your flake:

```nix
{
  inputs = {
    cgen = {
      url = "github:corbinwunderlich/cgen";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
}
```

...and use it as a package with:

```nix
inputs.cgen.packages.${pkgs.stdenv.hostPlatform.system}.default
```

Add it as a tarball to non-flake projects (not recommended):

```nix
let
  cgenSrc = builtins.fetchTarball {
    url = "https://github.com/corbinwunderlich/cgen";
    sha256 = "AAA..."; # find the real hash
  };

  cgen = import cgenSrc {};
in
```

Or install it imperatively:

```sh
nix profile add github:corbinwunderlich/cgen
```

# Configuration

cgen can be configured through a cgen configuration file, located in the current working directory. It can be YAML, JSON, TOML. Documentation for the config file is available [here](./CONFIG.md).

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

cgen will respect .ignore and .gitignore files unless told otherwise with command line flags. Flags can be viewed with `man cgen`, `--help`, or [here](./USAGE.md).

# Planned

- Bindings generation
- Rust frontend
