module completions {

  # A C/C++ header generator built in Rust
  export extern cgen [
    --config(-c): path        # Path to config file
    --watch(-w)               # Watch for changes in files/directories
    --watch-interval: string  # Polling interval for --watch, in milliseconds
    --ignore(-I)              # Respect .gitignore and .ignore when transforming directories (on by default)
    --hidden(-H)              # Allow transforming hidden files and directories (off by default)
    --help(-h)                # Print help
    --version(-V)             # Print version
    ...path: path             # Files or directories to transform
  ]

}

export use completions *
