package main

import (
	_ "embed"
	"github.com/rsteube/lazycomplete/internal/ps"
	"strings"
)

//go:embed internal/shell/bash.sh
var bash string

//go:embed internal/shell/elvish.elv
var elvish string

//go:embed internal/shell/fish.sh
var fish string

//go:embed internal/shell/xonsh.py
var xonsh string

//go:embed internal/shell/xonsh_header.py
var xonshHeader string

//go:embed internal/shell/oil.sh
var oil string

//go:embed internal/shell/powershell_header.ps1
var powershellHeader string

//go:embed internal/shell/powershell.ps1
var powershell string

//go:embed internal/shell/zsh.sh
var zsh string

func Fmt(completers map[string]string) string {
	header := ""
	body := ""
	switch ps.DetermineShell() {
	case "bash":
		body = bash
	case "elvish":
		body = elvish
	case "fish":
		body = fish
	case "oil":
		body = oil
	case "powershell":
		header = powershellHeader
		body = powershell
	case "xonsh":
        header= xonshHeader
		body = xonsh
	case "zsh":
		body = zsh
	}

	script := header
	for cmd, completer := range completers {
		out := body
		out = strings.Replace(out, "{{.CMD}}", cmd, -1)
		out = strings.Replace(out, "{{.COMPLETER}}", completer, -1)
		script = script + "\n" + out
	}
	return script
}
