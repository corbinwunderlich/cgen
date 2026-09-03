complete -c cgen -s c -l config -d 'Path to config file' -r -F
complete -c cgen -l watch-interval -d 'Polling interval for --watch, in milliseconds' -r
complete -c cgen -s w -l watch -d 'Watch for changes in files/directories'
complete -c cgen -s I -l ignore -d 'Respect .gitignore and .ignore when transforming directories (on by default)'
complete -c cgen -s H -l hidden -d 'Allow transforming hidden files and directories (off by default)'
complete -c cgen -s h -l help -d 'Print help'
complete -c cgen -s V -l version -d 'Print version'
