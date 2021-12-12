package main

import (
	"fmt"
	"os"
)

func main() {
	if len(os.Args)%2 != 1 {
		println("invalid amount of arguments")
		os.Exit(1)
	}

	completers := make(map[string]string, 0)
	for i := 1; i < len(os.Args)-1; i += 2 {
		completers[os.Args[i]] = os.Args[i+1]
	}
	fmt.Println(Fmt(completers))
}
