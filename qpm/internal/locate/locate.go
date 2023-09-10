package locate

import (
	"fmt"
	"os"
	"qpm/internal"
)

type ErrNotInitialised struct{}

func (e *ErrNotInitialised) Error() string {
	return "QPM is not initialised, please run qpm init"
}

type ErrNoSuchCacheFile struct{}

func (e *ErrNoSuchCacheFile) Error() string {
	return "The cache file does not exist"
}

func FindCacheFile(name string) ([]string, error) {
	filesFound := []string{}

	res, err := internal.IsInitialised()
	if err != nil {
		return nil, err
	}
	if !res {
		return nil, &ErrNotInitialised{}
	}
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return nil, err
	}

	qpmDir := homeDir + "/.qpm/"

	// Search local directory first

	localCacheDir := qpmDir + "local/cache/"
	localCacheFiles, err := os.ReadDir(localCacheDir)
	if err != nil {
		fmt.Println("Failed to read the directory", localCacheDir+":"+err.Error())
	}
	for _, f := range localCacheFiles {
		if !f.IsDir() || f.Name() == name {
			filesFound = append(filesFound, localCacheDir+name)
			break
		}
	}

	return filesFound, nil
}
