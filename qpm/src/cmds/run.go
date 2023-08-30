package cmds

import (
	"fmt"
	"os"
	"flag"
)

func Run() {
	fmt.Println("run command here")
	var quiz string
	flag.StringVar(&quiz, "q", "", "The quiz to run")
	flag.CommandLine.Parse(os.Args[2:])
}

func FindQuizPath(fname string) string {
	homeDir := GetHomeDir()
	ValidateQpmDirElseExit()

	// If a log file already exists, we will continue from it
	logPath := homeDir + ".qpm/local/log/" + fname + ".qqml.json"
	if DirExists(logPath) {
		return logPath
	}
	srcPath := homeDir + ".qpm/local/src" + fname + ".qqml"
	if DirExists(srcPath) {
		return srcPath
	}
	fmt.Println("Cannot find the qqml quiz: " + fname)
	os.Exit(1)
	return ""
}
