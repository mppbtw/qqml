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
	"github.com/spf13/cobra"
	"os"
)

var (
	rootCmd = &cobra.Command{
		Use:   "qpm",
		Short: "QPM is the package manager for QQML.",
		Long: `QPM is the QQML (Quiz Question Markup Language) package manager.
It is used to install QQML quizzes from either local files or remote repositories.
See <https://github.com/mrpiggypegasus/qqml> for more info.`,
	}
)

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		os.Exit(1)
	}
}

func init() {
	rootCmd.AddCommand()
}