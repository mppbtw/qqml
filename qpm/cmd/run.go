package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

var (
	fromFilePath string
	runCmd       = &cobra.Command{
		Use:   "run",
		Short: "Run an installed quiz",
		Long:  "Run an installed quiz from any of the available repos or locally installed files",
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println("placeholder")
		},
	}
)

func init() {
	runCmd.Flags().StringVarP(&fromFilePath, "file", "f", "", "from a file")
	rootCmd.AddCommand(runCmd)
}
