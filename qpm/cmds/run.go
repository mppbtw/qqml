//  QQML or the Quiz Question Markup Language.
//  Copyright (C) 2023 'MrPiggyPegasus'
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program. If not, see <https://www.gnu.org/licenses/>.

package cmds

import (
	"flag"
	"fmt"
	"os"
	"os/exec"
	"strings"
)

func Run() {
	var quiz string
	flag.StringVar(&quiz, "q", "", "The quiz to run")
	flag.CommandLine.Parse(os.Args[2:])
	if !IsFlagPassed("q") || quiz == "" {
		fmt.Println("The -q flag is required")
		os.Exit(1)
	}
	path, isLogFile := FindQuizPath(quiz)
	var cmd *exec.Cmd
	if isLogFile {
		cmd = exec.Command("qqml", "--resume", path, "--log", path)
	} else {
		logPath, err := logFileFromSrc(path)
		if err != nil {
			fmt.Println("Invalid path to the QQML file")
		}
		cmd = exec.Command("qqml", path, "--log", logPath)
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

type InvalidPathError struct{}

func (e InvalidPathError) Error() string {
	return "Invalid path to the source QQML file"
}
func logFileFromSrc(srcPath string) (string, error) {
	pathArr := strings.Split(srcPath, "/")
	for i := 0; i < len(pathArr); i++ {
		if pathArr[i] == "src" {
			pathArr[i] = "log"
			return strings.Join(pathArr, "/") + ".json", nil
		}
	}
	return "", InvalidPathError{}
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
