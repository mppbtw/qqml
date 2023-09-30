package qqml

import (
	"os"
	"os/exec"
)

type SrcType int8

const (
	QQMLFile SrcType = 0
	JsonFile SrcType = 1
)

type ErrSrcPathNotSpecified struct{}

func (e ErrSrcPathNotSpecified) Error() string {
	return "The SrcPath value has not been specified!"
}

type ErrNoSuchSrcType struct{}

func (e ErrNoSuchSrcType) Error() string {
	return "The specified SrcType value is out of bounds"
}

type QQMLRunCommand struct {
	srcPath string
	logPath string
	srcType SrcType
}

func (c *QQMLRunCommand) constructArgs() (string, error) {
	args := "run"

	if len(c.srcPath) == 0 {
		return "", ErrSrcPathNotSpecified{}
	}

	args += c.srcPath

	if c.srcType == JsonFile {
		args += "--json"
	} else if c.srcType != QQMLFile {
		return "", ErrNoSuchSrcType{}
	}

	if len(c.logPath) != 0 {
		args += "--log" + c.logPath
	}
	return args, nil
}

func (c *QQMLRunCommand) Output() (string, error) {
	args, err := c.constructArgs()
	if err != nil {
		return "", err
	}

	cmd := exec.Command("qqml", args)

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

	cmd := exec.Command("qqml", args)

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

	err = exec.Command("qqml", args).Run()
	if err != nil {
		return err
	}

	return nil
}
