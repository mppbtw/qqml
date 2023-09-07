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
	"github.com/spf13/cobra"
	"qpm/internal"
)

var initCmd = &cobra.Command{
	Use:   "init",
	Short: "Initialise the ~/.qpm directory.",
	Long:  "Initialise the ~/.qpm directory to store QQML quiz files",
	Run: func(cmd *cobra.Command, args []string) {
		if internal.IsInitialised() {
			fmt.Println("QPM is already initialised.")
			return
		}
	},
}

func init() {
	rootCmd.AddCommand(initCmd)
}
