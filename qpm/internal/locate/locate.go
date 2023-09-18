package locate

import (
	"fmt"
	"os"
	"qpm/internal"
	"strings"
)

type ErrNotInitialised struct{}

func (e *ErrNotInitialised) Error() string {
	return "QPM is not initialised, please run qpm init"
}

type ErrNoSuchCacheFile struct{}

func (e *ErrNoSuchCacheFile) Error() string {
	return "The cache file does not exist"
}

type ErrNoSuchLogFile struct{}

func (e *ErrNoSuchLogFile) Error() string {
	return "The cache file does not exist"
}

func FindCacheFile(name string) ([]string, error) {
	name = name + ".qqml.json"
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

	if len(filesFound) == 0 {
		return []string{}, &ErrNoSuchCacheFile{}
	}
	return filesFound, nil
}

func FindLogFile(name string) ([]string, error) {
	name = name + ".qqml.json"
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

	localLogDir := qpmDir + "local/cache/"
	localLogFiles, err := os.ReadDir(localLogDir)
	if err != nil {
		fmt.Println("Failed to read the directory", localLogDir+":"+err.Error())
	}
	for _, f := range localLogFiles {
		if !f.IsDir() || f.Name() == name {
			filesFound = append(filesFound, localLogDir+name)
			break
		}
	}

	if len(filesFound) == 0 {
		return []string{}, &ErrNoSuchLogFile{}
	}
	return filesFound, nil
}

func GenLogFileFromCache(cachePath string) string {
	pathSegments := strings.Split(cachePath, "/")
	pathSegments[len(pathSegments)-2] = "cache"
	return strings.Join(pathSegments, "/")
}
