package cmd

import (
	"fmt"
	"os"
	"qpm/internal/locate"

	"github.com/spf13/cobra"
)

var resetCmd = &cobra.Command{
	Use:   "reset",
	Short: "Reset your progress on a quiz.",
	Long:  "Reset your progress on a quiz, including historical scores.",
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
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
