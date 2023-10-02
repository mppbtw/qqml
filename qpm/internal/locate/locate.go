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

package locate

import (
	"fmt"
	"os"
	"qpm/internal/utils"
	"strings"
)

type ErrNotInitialised struct{}

func (e *ErrNotInitialised) Error() string {
	return "QPM is not initialised, please run qpm init"
}

func FindCacheFile(name string) ([]string, error) {
	name = name + ".qqml.json"
	filesFound := []string{}

	res, err := utils.IsInitialised()
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

func FindLogFile(name string) ([]string, error) {
	name = name + ".qqml.json"
	filesFound := []string{}

	res, err := utils.IsInitialised()
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
	localLogDir := qpmDir + "local/log/"
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

	return filesFound, nil
}

func GenLogFileFromCache(cachePath string) string {
	pathSegments := strings.Split(cachePath, "/")
	pathSegments[len(pathSegments)-2] = "log"
	return strings.Join(pathSegments, "/")
}
