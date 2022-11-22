_lazycomplete_{{.CMD}}() {
  unset -f _lazycomplete_{{.CMD}}
  source <({{.COMPLETER}})
  $"$(complete -p {{.CMD}} | sed -r 's/.* -F ([^ ]+).*/\1/')"
}
complete -F _lazycomplete_{{.CMD}} {{.CMD}}
