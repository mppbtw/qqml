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

package qqml

import (
	"os"
	"os/exec"
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

func (c *QQMLRunCommand) constructArgs() ([]string, error) {
	args := "run "

	if len(c.SrcPath) == 0 {
		return nil, ErrSrcPathNotSpecified{}
	}

	args += c.SrcPath

	if c.SrcType == JsonFile {
		args += " --json "
	} else if c.SrcType != QQMLFile {
		return nil, ErrNoSuchSrcType{}
	}

	if len(c.LogPath) != 0 {
		args += "--log " + c.LogPath
	}
	return strings.Split(args, " "), nil
}

func (c *QQMLRunCommand) Output() (string, error) {
	args, err := c.constructArgs()
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

func (c *QQMLRunCommand) Run() error {
	args, err := c.constructArgs()
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

func (c *QQMLRunCommand) RunHeadless() error {
	args, err := c.constructArgs()
	if err != nil {
		return err
	}

	err = exec.Command("qqml", args...).Run()
	if err != nil {
		return err
	}

	return nil
}
