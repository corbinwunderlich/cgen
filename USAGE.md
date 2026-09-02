# NAME

cgen

# SYNOPSIS

**cgen** \[**-c**\|**\--config**\] \[**-w**\|**\--watch**\]
\[**\--watch-interval**\] \[**-I**\|**\--ignore**\]
\[**-H**\|**\--hidden**\] \[**-v**\|**\--verbose**\]\...
\[**-q**\|**\--quiet**\]\... \[**-h**\|**\--help**\]
\[**-V**\|**\--version**\] \<*PATH*\>

# DESCRIPTION

# OPTIONS

**-c**, **\--config** *\<CONFIG\>*

:   Path to config file

**-w**, **\--watch**

:   Watch for changes in files/directories

**\--watch-interval** *\<WATCH_INTERVAL\>* \[default: 100\]

:   Polling interval for \--watch, in milliseconds

**-I**, **\--ignore**

:   Respect .gitignore and .ignore when transforming directories (on by
    default)

**-H**, **\--hidden**

:   Allow transforming hidden files and directories (off by default)

**-v**, **\--verbose**

:   Increase logging verbosity

**-q**, **\--quiet**

:   Decrease logging verbosity

**-h**, **\--help**

:   Print help

**-V**, **\--version**

:   Print version

\<*PATH*\>

:   Files or directories to transform

# VERSION

v0.1.0
