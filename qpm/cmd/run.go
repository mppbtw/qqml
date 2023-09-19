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
	"fmt"
	"os"
	"os/exec"
	"qpm/internal"
	"qpm/internal/locate"

	"github.com/spf13/cobra"
)

var (
	runCmd = &cobra.Command{
		Use:     "run",
		Short:   "Run a quiz",
		Long:    "Run a quiz from any of the available repos or locally installed files",
		Aliases: []string{"r"},
		Args:    cobra.ExactArgs(1),
		Run: func(cmd *cobra.Command, args []string) {

			// Check that QPM is in a valid state
			res, err := internal.IsInitialised()
			if !res {
				fmt.Println("QPM is not initialised, please run qpm init")
				os.Exit(1)
			}
			if err != nil {
				fmt.Println("Failed to tell if QPM is initialised:", err.Error())
				os.Exit(1)
			}

			quiz := args[0]

			paths, err := locate.FindLogFile(quiz)
			if err != nil {
				paths, err := locate.FindCacheFile(quiz)
				if err != nil {
					fmt.Println("No such quiz", quiz)
					os.Exit(1)
				}
				if len(paths) != 1 {
					fmt.Println("Multiple quizzes found with that name")
					os.Exit(1)
				}
				command := exec.Command("qqml",
					"--resume",
					paths[0],
					"--log",
					locate.GenLogFileFromCache(paths[0]),
				)
				command.Stdout = os.Stdout
				command.Stderr = os.Stderr
				command.Stdin = os.Stdin
				command.Run()
				os.Exit(0)
			}
			if len(paths) != 1 {
				fmt.Println("Multiple quizzes found with that name")
				os.Exit(1)
			}
			fmt.Println("logfile path: ", paths[0])

			command := exec.Command("qqml",
				"--resume",
				paths[0],
				"--log",
				paths[0],
			)
			command.Stdout = os.Stdout
			command.Stderr = os.Stderr
			command.Stdin = os.Stdin
			command.Run()
		},
	}
)

func init() {
	rootCmd.AddCommand(runCmd)
}
