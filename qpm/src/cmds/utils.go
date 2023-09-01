package cmds

import (
	"errors"
	"fmt"
	"os"
	"flag"
)

func DirExists(path string) bool {
	_, err := os.Stat(path)
	return !errors.Is(err, os.ErrNotExist)
}

func ValidateQpmDirElseExit() {
	dirExistsWithWarningAndExit(".qpm")
	dirExistsWithWarningAndExit(".qpm/local")
	dirExistsWithWarningAndExit(".qpm/local/src")
	dirExistsWithWarningAndExit(".qpm/local/log")
	dirExistsWithWarningAndExit(".qpm/repos")
}

func GetHomeDir() string {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		fmt.Println("Failed to get your home directory, are you root?")
	}
	return homeDir + "/"
}

func dirExistsWithWarningAndExit(path string) {
	homeDir := GetHomeDir()
	if !DirExists(homeDir + path) {
		fmt.Println("The directory " + homeDir + path + " does not exist")
		fmt.Println("Please run qpm --init first")
		os.Exit(1)
	}
}

func IsFlagPassed(name string) bool {
    found := false
    flag.Visit(func(f *flag.Flag) {
        if f.Name == name {
            found = true
        }
    })
    return found
}

func FileExists(filename string) bool {
    info, err := os.Stat(filename)
    if os.IsNotExist(err) {
        return false
    }
    return !info.IsDir()
}

func CopyFile(in, out string) (int64, error) {
   i, err := os.Open(in)
   if err != nil { return 0, err }
   defer i.Close()

   o, err := os.Create(out)
   if err != nil { return 0, err }
   defer o.Close()

   return o.ReadFrom(i)
}
