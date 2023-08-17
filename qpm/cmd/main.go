package main

import (
	"fmt"
	"github.com/mrpiggypegasus/qqml/internal/argparsing"
)

func main() {
	switch internal.NextArg() {
	case "":
		fmt.Println("No arguments supplied. Run qpm --help for more info")
	case "version":
		versionMessage()
	default:
		helpScreen()
	}
}

func helpScreen() {
	fmt.Println("QPM v1.0 (c) 2023 'MrPiggyPegasus'")
}

func versionMessage() {
	fmt.Println("QPM v1.0 (c) 2023 'MrPiggyPegasus'",)
}
