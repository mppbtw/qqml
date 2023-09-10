package cmd

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

var (
	fromFilePath string
	runCmd       = &cobra.Command{
		Use:   "run",
		Short: "Run a quiz",
		Long:  "Run a quiz from any of the available repos or locally installed files",
		Run: func(cmd *cobra.Command, args []string) {
			if fromFilePath == "" {
				fmt.Println("placeholder")
				os.Exit(1)
			}
		},
	}
)

func init() {
	runCmd.Flags().StringVarP(&fromFilePath, "file", "f", "", "from a file")
	rootCmd.AddCommand(runCmd)
}
