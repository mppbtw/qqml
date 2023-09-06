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

package main

import (
	"fmt"
	"os"
	"qpm/cmds"
)

func main() {
	args := os.Args[1:]
	if len(args) == 0 {
		fmt.Println("No command specified, use --help for more info")
		os.Exit(1)
	}
	switch args[0] {
	case "init":
		cmds.Init()
	case "run":
		cmds.Run()
	case "install":
		cmds.Install()
	default:
		os.Exit(1)
	}
}
