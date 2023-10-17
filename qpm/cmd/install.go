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

package cmd

import (
	"bufio"
	"fmt"
	"os"
	"qpm/internal/argparse"
	"qpm/internal/qqml"
	"qpm/internal/utils"
	"strings"
	"unicode"
)

var (
	installCmd = argparse.Command{
		Usage: "install",
		Short: "Install QQML quizzes",
		Long:  "Install QQML quizzes from either local files or remote repos",
		Run: func(args []string, flags argparse.AnsweredFlags) {
			fileFlag, err := flags.Get("--file")
			if err != nil {
				fmt.Println("Missing the --file flag!")
				os.Exit(1)
			}
			filePath := fileFlag.Arg.StringArg

			if !utils.PathExists(filePath) {
				fmt.Println("File", filePath, "does not exist")
				os.Exit(1)
			}
			if res, err := utils.IsInitialised(); !(res && err == nil) {
				fmt.Println("QPM has not been initialised, please run qpm init")
				os.Exit(1)
			}

			homeDir, err := os.UserHomeDir()
			if err != nil {
				fmt.Println("Failed to get your home directory:", err.Error())
				os.Exit(1)
			}

			qpmDir := homeDir + "/.qpm/"
			srcContents, err := os.ReadFile(filePath)
			if err != nil {
				fmt.Println("Failed to read the file", filePath+":", err.Error())
			}

			var name string
			for {
				name = ""
				reader := bufio.NewReader(os.Stdin)
				fmt.Print("Please enter the name of the quiz: ")
				name, err = reader.ReadString('\n')
				name = strings.TrimSuffix(name, "\n")
				if err != nil {
					fmt.Println("Failed to read from stdin")
					os.Exit(1)
				}

				// Validate the name
				if strings.Contains(name, "/") || strings.Contains(name, "\\") {
					fmt.Printf("The name must contain no slashes\n\n")
					continue
				}
				if len(name) == 0 {
					fmt.Printf("The name must not be empty\n\n")
					continue
				}
				if utils.PathExists(qpmDir + "local/src/" + name + ".qqml") {
					fmt.Printf("A quiz of that name already exists\n\n")
					continue
				}

				isAscii := true
				for _, ch := range name {
					if ch > unicode.MaxASCII {
						isAscii = false
					}
				}

				if !isAscii {
					fmt.Printf("Only ASCII characters are allowed in the name\n\n")
					continue
				}
				break
			}

			if err := os.WriteFile(qpmDir+"local/src/"+name+".qqml", srcContents, os.FileMode(0644)); err != nil {
				fmt.Println("Failed to install quiz:", err.Error())
				os.Exit(1)
			}

			// Compile and cache the JSON
			cacheFilePath := qpmDir + "local/cache/" + name + ".qqml.json"
			if utils.PathExists(cacheFilePath) {
				fmt.Println("The JSON cache file", cacheFilePath, "already exists, removing it")
				err := os.Remove(cacheFilePath)
				if err != nil {
					fmt.Println("Failed to remove", cacheFilePath+":", err.Error())
					os.Exit(1)
				}

			}

			err = qqml.CompileQQMLToFile(qpmDir+"local/src/"+name+".qqml", cacheFilePath)
			if err != nil {
				fmt.Println("Error during compilation: ", err.Error())
				os.Exit(1)
			}
		},
	}
)

func init() {
	rootCmd.AddCommand(installCmd)
}
