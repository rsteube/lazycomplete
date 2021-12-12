function _lazycomplete_{{.CMD}}
   functions --erase _lazycomplete_{{.CMD}}
   complete -c '{{.CMD}}' --erase
   {{.COMPLETER}} | source
   complete --do-complete=(commandline -cp)
end
complete -c '{{.CMD}}' --erase
complete -c '{{.CMD}}' -f -a '(_lazycomplete_{{.CMD}} {{.CMD}})'
