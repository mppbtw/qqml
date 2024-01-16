//  QQML or the Quiz Question Markup Language.
//  Copyright (C) 2023 'mppbtw'
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

package qqml

import (
	"os"
	"os/exec"
	"qpm/internal/utils"
	"strings"
)

type SrcType int8

const (
	QQMLFile SrcType = 0
	JsonFile SrcType = 1
)

type QQMLRunCommand struct {
	SrcPath string
	LogPath string
	SrcType SrcType
}

type ErrNoSuchFileOrDirectory struct{}

func (e ErrNoSuchFileOrDirectory) Error() string {
	return "No such file or directory (os error 2)"
}

func (self *QQMLRunCommand) validateArgs() error {
	if !utils.PathExists(self.SrcPath) {
		return ErrNoSuchFileOrDirectory{}
	}
	return nil
}

func (self *QQMLRunCommand) constructArgs() ([]string, error) {
	if err := self.validateArgs(); err != nil {
		return []string{}, err
	}
	args := "run "

	if len(self.SrcPath) == 0 {
		return nil, ErrSrcPathNotSpecified{}
	}

	args += self.SrcPath

	if self.SrcType == JsonFile {
		args += " --json "
	} else if self.SrcType != QQMLFile {
		return nil, ErrNoSuchSrcType{}
	}

	if len(self.LogPath) != 0 {
		args += "--log " + self.LogPath
	}
	return strings.Split(args, " "), nil
}

func (self *QQMLRunCommand) Output() (string, error) {
	args, err := self.constructArgs()
	if err != nil {
		return "", err
	}

	cmd := exec.Command("qqml", args...)

	out, err := cmd.Output()

	if err != nil {
		return "", err
	}

	return string(out), nil
}

func (self *QQMLRunCommand) Run() error {
	args, err := self.constructArgs()
	if err != nil {
		return err
	}

	cmd := exec.Command("qqml", args...)

	cmd.Stdout = os.Stdout
	cmd.Stdin = os.Stdin
	cmd.Stderr = os.Stderr

	err = cmd.Run()
	if err != nil {
		return err
	}

	return nil
}

func (self *QQMLRunCommand) RunHeadless() error {
	args, err := self.constructArgs()
	if err != nil {
		return err
	}

	err = exec.Command("qqml", args...).Run()
	if err != nil {
		return err
	}

	return nil
}
n nil
}
