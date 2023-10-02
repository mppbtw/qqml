package qqml

import (
	"os/exec"
)

func CompileQQML(srcPath string) (string, error) {
	cmd := exec.Command("qqml", "compile", srcPath)

	out, err := cmd.Output()
	return string(out), err
}

func CompileQQMLToFile(srcPath, outPath string) error {
	cmd := exec.Command("qqml", "compile", srcPath, "--output", outPath)

	return cmd.Run()
}
