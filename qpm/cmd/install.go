package cmd

import (
	"fmt"
	"os"
	"qpm/internal"
	"strings"

	"github.com/spf13/cobra"
)

var (
	filePath   string
	installCmd = &cobra.Command{
		Use:     "install",
		Short:   "Install QQML quizzes",
		Long:    "Install QQML quizzes from either local files or remote repos",
		Aliases: []string{"i"},
		Run: func(cmd *cobra.Command, args []string) {
			if !internal.PathExists(filePath) {
				fmt.Println("File", filePath, "does not exist")
				os.Exit(1)
			}
			if res, err := internal.IsInitialised(); !(res && err == nil) {
				fmt.Println("QPM has not been initialised, please run qpm init")
				os.Exit(1)
			}

			pathSegments := strings.Split(filePath, "/")
			fileName := pathSegments[len(pathSegments)-1]
			homeDir, err := os.UserHomeDir()
			if err != nil {
				fmt.Println("Failed to get your home directory:", err.Error())
				os.Exit(1)
			}

			qpmDir := homeDir + "/.qpm/"
			srcContents, err := os.ReadFile(filePath)
			if err != nil {
				fmt.Println("Failed to read the file", filePath+":", err.Error())
			}

			os.WriteFile(qpmDir+fileName, srcContents, os.FileMode(0644))
		},
	}
)
func init() {
	installCmd.Flags().StringVarP(&filePath, "file", "f", "", "install from local file")
	installCmd.MarkFlagFilename("file")
	installCmd.MarkFlagRequired("file")

	rootCmd.AddCommand(installCmd)
}
