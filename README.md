# About cgen

cgen is a C/C++ header generator built in Rust. It supports almost all of C, even C23, due to it being built around Libclang.

# Installation

The recommended installation is with Nix. Add it as an input to your devshells, or install it imperatively with

```sh
nix profile add github:corbinwunderlich/cgen
```

# Planned

- Bindings generation
- Rust frontend
