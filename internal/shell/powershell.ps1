Function _lazycomplete_{{.CMD}} {
    param($wordToComplete, $commandAst, $cursorPosition)
    Register-ArgumentCompleter -Native -CommandName '{{.CMD}}' -ScriptBlock {}
    {{.COMPLETER}} | out-string | invoke-expression
    [System.Management.Automation.CommandCompletion]::CompleteInput($commandAst.ToString().PadRight($cursorPosition, ' ').Substring(0, $cursorPosition), $cursorPosition, $null).CompletionMatches
}
Register-ArgumentCompleter -Native -CommandName '{{.CMD}}' -ScriptBlock (Get-Item "Function:_lazycomplete_{{.CMD}}").ScriptBlock

