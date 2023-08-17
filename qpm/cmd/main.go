package main

import (
	"fmt"
	"github.com/mrpiggypegasus/qqml/internal/argparsing"
	"os"
)

func main() {
	switch internal.NextArg() {
	case "":
		fmt.Println("No arguments supplied. Run qpm --help for more info")
	case "version":
		versionMessage()
	case "init":
		initConfig()
	default:
		helpScreen()
	}
}

func helpScreen() {
	fmt.Println("QPM v1.0 (c) 2023 'MrPiggyPegasus'")
}

func versionMessage() {
	fmt.Println("QPM v1.0 (c) 2023 'MrPiggyPegasus'")
}

func initConfig() {
	verbose := true;

	homeDir, err := os.UserHomeDir();
	if err != nil {
		fmt.Println("Failed to get user home directory. Are you root?")
		return
	}
	homeDir = homeDir + "/"

	if doesPathExist(homeDir + ".qpm/") {
		if verbose {
			fmt.Println("~/.qpm/ already exists.")
		}
	} else {
		if os.Mkdir(homeDir + ".qpm/", 0755) != nil {
			fmt.Fprintf(os.Stderr, "Failed to create directory ~/.qpm\n")
			return
		}
	}

	if doesPathExist(homeDir + ".qpm/repos.cfg") {
		if verbose {
			fmt.Println("~/.qpm/repos.cfg already exists.")
		}
	} else {
		_, err := os.Create(homeDir + ".qpm/repos.cfg")
		if err != nil {
			fmt.Fprintf(os.Stderr, "Failed to create file ~/.qpm/repos.cfg\n")
			return
		}
	}

	if doesPathExist(homeDir + ".qpm/local/") {
		if verbose {
			fmt.Println("~/.qpm/local/ already exists.")
		}
	} else {
		if os.Mkdir(homeDir + ".qpm/local/", 0755) != nil {
			fmt.Fprintf(os.Stderr, "Failed to create directory ~/.qpm/local/\n")
			return
		}
	}
}

func doesPathExist(path string) bool {
	_, err := os.Stat(path);
	return err == nil
}
