_lazycomplete_{{.CMD}}() {
  unset -f _lazycomplete_{{.CMD}}
  source <({{.COMPLETER}})
  $"$(complete -p {{.CMD}} | awk '{print $3}')"
}
complete -F _lazycomplete_{{.CMD}} {{.CMD}}
