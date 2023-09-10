package cmd

import (
	"fmt"
	"os"
	"os/exec"
	"qpm/internal"
	"qpm/internal/locate"

	"github.com/spf13/cobra"
)

var (
	fromFilePath string
	runCmd       = &cobra.Command{
		Use:   "run",
		Short: "Run a quiz",
		Long:  "Run a quiz from any of the available repos or locally installed files",
		Run: func(cmd *cobra.Command, args []string) {
			res, err := internal.IsInitialised()
			if !res {
				fmt.Println("QPM is not initialised, please run qpm init")
				os.Exit(1)
			}
			if err != nil {
				fmt.Println("Failed to tell if QPM is initialised:", err.Error())
				os.Exit(1)
			}

			if fromFilePath == "" {
				files, err := locate.FindCacheFile(fromFilePath)
				if err != nil {
					fmt.Println("Failed to locate that quiz:", err.Error())
				}
				if len(files) != 1 {
					fmt.Println("Multiple files found, placeholder")
					os.Exit(1)
				}
				file := files[0]
				fmt.Println("file: ", file)
			} else {
				cmd := exec.Command("qqml", fromFilePath)
				cmd.Stdout = os.Stdout
				cmd.Stdin = os.Stdin
				err := cmd.Run()
				if err != nil {
					fmt.Println(err.Error())
				}
			}
		},
	}
)

func init() {
	runCmd.Flags().StringVarP(&fromFilePath, "file", "f", "", "from a file")
	rootCmd.AddCommand(runCmd)
}
