package cmds

import (
	"fmt"
	"os"
	"os/exec"
)

func Run(quiz string) {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		fmt.Println("Failed to get the user home directory. Are you root?")
		os.Exit(1)
	}
	path := homeDir + "/.qpm/local/" + quiz
	log_path := path + ".json"

	if !pathExists(homeDir + "/.qpm/local/") {
		fmt.Println("Error: ~/.qpm/local does not exist. Please run qpm init.")
		os.Exit(1)
	}

	if !pathExists(path) {
		fmt.Printf("Error: no such QQML package: '%s'\n", quiz)
		os.Exit(1)
	}

	cmd := exec.Command("qqml", path, "-l", log_path)
	cmd.Stderr = os.Stderr
	cmd.Stdout = os.Stdout
	cmd.Stdin = os.Stdin

	err = cmd.Run()
	if err != nil {
		fmt.Println("QQML subprocess crashed!")
		fmt.Println()
		os.Exit(1)
		return
	}
}

func pathExists(path string) bool {
	_, err := os.Stat(path)
	return err == nil
}
