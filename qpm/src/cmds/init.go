package cmds

import (
	"fmt"
	"os"
)

func Init() {
	// The structure of the qpm directory is as follows:
	//
	// ~/.qpm/
	//       local
	//            log
	//            src
	//       repos

	ensureExists(".qpm/")
	ensureExists(".qpm/local/")
	ensureExists(".qpm/local/log/")
	ensureExists(".qpm/local/src/")
	ensureExists(".qpm/repos/")
}

func ensureExists(path string) {
	homeDir, err := os.UserHomeDir()
	homeDir += "/"
	if err != nil {
		fmt.Println("Failed to retrieve you home directory. Are you root?")
	}

	if DirExists(homeDir + path) {
		fmt.Println("The directory " + homeDir + path + " already exists")
	} else {
		if err := os.Mkdir(homeDir + path, os.FileMode(0777)); err != nil {
			fmt.Println("Failed to create the directory " + homeDir + path)
			fmt.Println(err)
			os.Exit(1)
		} else {
			fmt.Println("Created the directory " + homeDir + path)
		}
	}
}
