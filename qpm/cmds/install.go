package cmds

import (
	"flag"
	"fmt"
	"os"
	"strings"
)

func Install() {
	var quiz string
	flag.StringVar(&quiz, "q", "", "The quiz to Install")
	flag.CommandLine.Parse(os.Args[2:])
	if !IsFlagPassed("q") || quiz == "" {
		fmt.Println("The -q flag is required")
		os.Exit(1)
	}
	if FileExists(quiz) {
		quizFileName := strings.Split(quiz, "/")[len(strings.Split(quiz, "/"))-1]
		if len(strings.Split(quizFileName, ".")) == 3 {
			fmt.Println("quizFileName names must only contain one dot")
			os.Exit(1)
		}
		if len(strings.Split(quizFileName, ".")) == 1 {
			fmt.Println("quizFileName names must have the .qqml file extension<F4>")
			os.Exit(1)
		}
		if strings.Split(quizFileName, ".")[1] != "qqml" {
			fmt.Println("quizFileName names must have the .qqml file extension<F4>")
			os.Exit(1)
		}

		// Everything has been validated
		destPath := GetHomeDir() + ".qpm/local/src/" + quizFileName
		_, err := CopyFile(quiz, destPath)
		if err != nil {
			fmt.Println("Error during file copying:", err)
			os.Exit(1)
		}
	}
}
