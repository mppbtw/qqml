package cmds

import (
	"flag"
	"fmt"
	"os"
	"os/exec"
)

func Run() {
	var quiz string
	flag.StringVar(&quiz, "q", "", "The quiz to run")
	flag.CommandLine.Parse(os.Args[2:])
	if !isFlagPassed("q") {
		fmt.Println("The -q flag is required")
		os.Exit(1)
	}
	path, isLogFile := FindQuizPath(quiz)
	var cmd *exec.Cmd
	if isLogFile {
		cmd = exec.Command("qqml", "--resume", path, "-log", path)
	} else {
		cmd = exec.Command("qqml", path)
	}

	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err := cmd.Run()
	if err != nil {
		fmt.Println("Executing qqml failed with error: ", err)
	}
	os.Exit(0)
}

func FindQuizPath(fname string) (string, bool) {
	homeDir := GetHomeDir()
	ValidateQpmDirElseExit()

	// If a log file already exists, we will continue from it
	logPath := homeDir + ".qpm/local/log/" + fname + ".qqml.json"
	if DirExists(logPath) {
		return logPath, true
	}
	srcPath := homeDir + ".qpm/local/src/" + fname + ".qqml"
	if DirExists(srcPath) {
		return srcPath, false
	}
	fmt.Println("Cannot find the qqml quiz: " + fname)
	os.Exit(1)
	return "", false
}



func isFlagPassed(name string) bool {
    found := false
    flag.Visit(func(f *flag.Flag) {
        if f.Name == name {
            found = true
        }
    })
    return found
}
