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
	"qpm/internal/utils"
)

var initCmd = argparse.Command{
	Usage: "init",
	Short: "Initialise the ~/.qpm directory",
	Long:  "Initialise the ~/.qpm directory to store QQML quiz files",
	Run: func(_ []string, _ argparse.AnsweredFlags) {
		res, err := utils.IsInitialised()
		if err != nil {
			fmt.Println("Failed to tell if QPM is already initialised:", err.Error())
			os.Exit(1)
		}
		if res {
			fmt.Println("QPM is already initialised.")
			os.Exit(0)
		}

		requiredDirs := [6]string{"", "local", "local/src", "local/log", "local/cache", "repos"}
		homeDir, err := os.UserHomeDir()
		if err != nil {
			fmt.Println("Failed to get your home directory:", err.Error())
			os.Exit(1)
		}
		qpmDir := homeDir + "/.qpm/"
		for _, dir := range requiredDirs {
			dir = qpmDir + dir
			if utils.PathExists(dir) {
				fmt.Println("The directory", dir, "already exists")
				continue
			}
			err := os.Mkdir(dir, os.FileMode(0777))
			if err != nil {
				fmt.Println("Failed to create the directory", dir+":", err.Error())
				os.Exit(1)
			}
			fmt.Println("Successfully created the directory", dir)
		}
	},
}

func init() {
	rootCmd.AddCommand(initCmd)
}
