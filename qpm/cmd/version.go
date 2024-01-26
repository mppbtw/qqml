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
	"qpm/internal/argparse"
)

func init() {
	rootCmd.AddCommand(versionCmd)
}

var versionCmd = argparse.Command{
	Usage: "version",
	Short: "Print the current QPM version number",
	Long:  "Take this number with a grain of salt, I'm forgetful sometimes",
	Run: func(_ []string, _ argparse.AnsweredFlags) {
		fmt.Println("This installation of QPM is from QQML v0.5.1")
	},
}
