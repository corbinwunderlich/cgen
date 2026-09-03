
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'cgen' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'cgen'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'cgen' {
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Path to config file')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'Path to config file')
            [CompletionResult]::new('--watch-interval', '--watch-interval', [CompletionResultType]::ParameterName, 'Polling interval for --watch, in milliseconds')
            [CompletionResult]::new('-w', '-w', [CompletionResultType]::ParameterName, 'Watch for changes in files/directories')
            [CompletionResult]::new('--watch', '--watch', [CompletionResultType]::ParameterName, 'Watch for changes in files/directories')
            [CompletionResult]::new('-I', '-I ', [CompletionResultType]::ParameterName, 'Respect .gitignore and .ignore when transforming directories (on by default)')
            [CompletionResult]::new('--ignore', '--ignore', [CompletionResultType]::ParameterName, 'Respect .gitignore and .ignore when transforming directories (on by default)')
            [CompletionResult]::new('-H', '-H ', [CompletionResultType]::ParameterName, 'Allow transforming hidden files and directories (off by default)')
            [CompletionResult]::new('--hidden', '--hidden', [CompletionResultType]::ParameterName, 'Allow transforming hidden files and directories (off by default)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
