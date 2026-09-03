```
A C/C++ header generator built in Rust

Usage: cgen [OPTIONS] <PATH>...

Arguments:
  <PATH>...  Files or directories to transform

Options:
  -c, --config <CONFIG>
          Path to config file
  -w, --watch
          Watch for changes in files/directories
      --watch-interval <WATCH_INTERVAL>
          Polling interval for --watch, in milliseconds [default: 100]
  -I, --ignore
          Respect .gitignore and .ignore when transforming directories (on by default)
  -H, --hidden
          Allow transforming hidden files and directories (off by default)
  -h, --help
          Print help
  -V, --version
          Print version
```
