package cmds

import (
	"errors"
	"fmt"
	"os"
)

func DirExists(path string) bool {
	_, err := os.Stat(path)
	return !errors.Is(err, os.ErrNotExist)
}

func ValidateQpmDirElseExit() {
	dirExistsWithWarningAndExit(".qpm")
	dirExistsWithWarningAndExit(".qpm/local")
	dirExistsWithWarningAndExit(".qpm/local/src")
	dirExistsWithWarningAndExit(".qpm/local/log")
	dirExistsWithWarningAndExit(".qpm/repos")
}

func GetHomeDir() string {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		fmt.Println("Failed to get your home directory, are you root?")
	}
	return homeDir + "/"
}

func dirExistsWithWarningAndExit(path string) {
	homeDir := GetHomeDir()
	if !DirExists(homeDir + path) {
		fmt.Println("The directory " + homeDir + path + " does not exist")
		fmt.Println("Please run qpm --init first")
		os.Exit(1)
	}
}
