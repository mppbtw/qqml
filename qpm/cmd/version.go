package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

func init() {
	rootCmd.AddCommand(versionCmd)
}

var versionCmd = &cobra.Command{
	Use:   "version",
	Short: "Print the current QPM version number",
	Long:  "Take this number with a grain of salt, I'm forgetful sometimes",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("This installation of QPM is from QQML v0.4.1")
	},
}
