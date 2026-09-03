
use builtin;
use str;

set edit:completion:arg-completer[cgen] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'cgen'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'cgen'= {
            cand -c 'Path to config file'
            cand --config 'Path to config file'
            cand --watch-interval 'Polling interval for --watch, in milliseconds'
            cand -w 'Watch for changes in files/directories'
            cand --watch 'Watch for changes in files/directories'
            cand -I 'Respect .gitignore and .ignore when transforming directories (on by default)'
            cand --ignore 'Respect .gitignore and .ignore when transforming directories (on by default)'
            cand -H 'Allow transforming hidden files and directories (off by default)'
            cand --hidden 'Allow transforming hidden files and directories (off by default)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
    ]
    $completions[$command]
}
