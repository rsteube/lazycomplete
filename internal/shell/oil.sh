_lazycomplete_{{.CMD}}() {
  unset -f _lazycomplete_{{.CMD}}
  source <({{.COMPLETER}})
  $"$(complete | grep --only-matching "^{{.CMD}} .*ShellFuncAction [^>]\+" | awk '{print $5}')"
}
complete -F _lazycomplete_{{.CMD}} {{.CMD}}

