package main

import (
	"os"
	"qpm/src/cmds"
	"fmt"
)

func main() {
	args := os.Args[1:]
	if len(args) == 0 {
		fmt.Println("No command specified, use --help for more info")
		os.Exit(1)
	}

	switch args[0] {

	case "init":
		cmds.Init()
	case "run":
		cmds.Run()
	case "install":
		cmds.Install()
	default:
		os.Exit(1)
	}
}
