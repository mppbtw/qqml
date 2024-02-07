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
)

var resetCmd = argparse.Command{
	Usage: "reset",
	Short: "Reset your progress on a quiz.",
	Long:  "Reset your progress on a quiz, including historical scores.",
	Args:  argparse.ExactArgs(1),
	Run: func(args []string, flags argparse.AnsweredFlags) {
		quiz := args[0]

		logFiles, err := locate.FindLogFile(quiz)
		if err != nil {
			fmt.Println("Failed to locate quiz", quiz+": "+err.Error())
			os.Exit(1)
		}

		if len(logFiles) > 1 {
			fmt.Println("The quiz name", quiz, "is amgiuous.")
			os.Exit(1)
		}

		err = os.Remove(logFiles[0])
		if err != nil {
			fmt.Println("Failed to remove file", logFiles[0]+": ", err.Error())
			os.Exit(1)
		}
	},
}

func init() {
	rootCmd.AddCommand(resetCmd)
}
