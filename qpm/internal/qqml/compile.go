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
