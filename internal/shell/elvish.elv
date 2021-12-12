set edit:completion:arg-completer[{{.CMD}}] = {|@arg|
      set edit:completion:arg-completer[{{.CMD}}] = {|@arg|}
      eval ({{.COMPLETER}} | slurp)
      $edit:completion:arg-completer[{{.CMD}}] $@arg
}
