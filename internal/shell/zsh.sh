function _lazycomplete_{{.CMD}} {
    compdef -d {{.CMD}}
    unfunction _lazycomplete_{{.CMD}}
    source <({{.COMPLETER}})
}
compdef _lazycomplete_{{.CMD}} {{.CMD}}

