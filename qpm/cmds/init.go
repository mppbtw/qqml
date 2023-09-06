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
	"fmt"
	"os"
)

func Init() {
	// The structure of the qpm directory is as follows:
	//
	// ~/.qpm/
	//       local
	//            log
	//            src
	//       repos
	ensureExists(".qpm/")
	ensureExists(".qpm/local/")
	ensureExists(".qpm/local/log/")
	ensureExists(".qpm/local/src/")
	ensureExists(".qpm/repos/")
}
func ensureExists(path string) {
	homeDir, err := os.UserHomeDir()
	homeDir += "/"
	if err != nil {
		fmt.Println("Failed to retrieve you home directory. Are you root?")
	}
	if DirExists(homeDir + path) {
		fmt.Println("The directory " + homeDir + path + " already exists")
	} else {
		if err := os.Mkdir(homeDir+path, os.FileMode(0777)); err != nil {
			fmt.Println("Failed to create the directory " + homeDir + path)
			fmt.Println(err)
			os.Exit(1)
		} else {
			fmt.Println("Created the directory " + homeDir + path)
		}
	}
}
