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
