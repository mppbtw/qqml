//  QQML or the Quiz Question Markup Language.
//  Copyright (C) 2023 'mppbtw'
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
	"fmt"
	"os"
	"qpm/internal/argparse"
	"qpm/internal/locate"
	"qpm/internal/qqml"
	"qpm/internal/utils"
)

var (
	runCmd = argparse.Command{
		Usage: "run",
		Short: "Run a quiz",
		Long:  "Run a quiz from any of the available repos or locally installed files",
		Args:  argparse.ExactArgs(1),
		Run: func(args []string, flags argparse.AnsweredFlags) {
			if fileFlag, err := flags.Get("--file"); err == nil {
				command := qqml.QQMLRunCommand{
					SrcPath: fileFlag.Arg.StringArg,
				}
				if err = command.Run(); err != nil {
					fmt.Println("Failed to run the quiz:", err.Error())
					os.Exit(1)
				}
				os.Exit(0)
			}

			// Check that QPM is in a valid state
			res, err := utils.IsInitialised()
			if !res {
				fmt.Println("QPM is not initialised, please run qpm init")
				os.Exit(1)
			}
			if err != nil {
				fmt.Println("Failed to tell if QPM is initialised:", err.Error())
				os.Exit(1)
			}

			quiz := args[0]
			logPath, err := locate.FindLogFile(quiz)
			if err != nil {
				fmt.Println("Error locating the quiz:", err)
				os.Exit(1)
			}

			// We know there is a log file
			if len(logPath) != 0 {
				if len(logPath) > 1 {
					fmt.Println("Many quizzes found by the name:", quiz)
					os.Exit(1)
				}

				command := qqml.QQMLRunCommand{}
				command.SrcType = qqml.JsonFile
				command.SrcPath = logPath[0]
				command.LogPath = logPath[0]
				if err = command.Run(); err != nil {
					fmt.Println("Failed to run the quiz:", err.Error())
					os.Exit(1)
				}

			} else {
				// We know there is no log file and we have to use the cache
				cachePaths, err := locate.FindCacheFile(quiz)
				if err != nil {
					fmt.Println("Error locating the quiz:", err)
					os.Exit(1)
				}

				// There is no log file so there not being a cache means the quiz
				// doesn't exist because the cache should be created upon installation
				if len(cachePaths) == 0 {
					fmt.Println("Failed to find the quiz:", quiz)
					os.Exit(1)
				}

				// There will be a process in the future for selecting one of these
				if len(cachePaths) > 1 {
					fmt.Println("Many quizzes found by the name:", quiz)
					os.Exit(1)
				}

				// We can now be sure that the cache file has been found
				command := qqml.QQMLRunCommand{}
				command.SrcType = qqml.JsonFile
				command.SrcPath = cachePaths[0]
				command.LogPath = locate.GenLogFileFromCache(cachePaths[0])
				if err = command.Run(); err != nil {
					fmt.Println("Failed to run the quiz:", err.Error())
					os.Exit(1)
				}
			}

			os.Exit(0)
		},
	}
)

func init() {
	runCmd.AddFlag(argparse.Flag{
		Usage:                    "--file",
		Long:                     "Run from a local file (if you want any persistency, please install the file as a package)",
		Aliases:                  []string{"-f"},
		Arg:                      argparse.StringFlagArgumentType,
		Required:                 false,
		ArgumentCountsForCommand: true,
	})
	rootCmd.AddCommand(runCmd)
}
