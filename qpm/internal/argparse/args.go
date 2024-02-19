//  QQML or the Quiz Question Markup Language.
//  Copyright (C) 2024 'mppbtw'
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

package argparse

import "fmt"

type ExpectedArgs interface {
	Validate([]string) bool
	ToString() string
}
type exactArgs struct {
	num int
}
type maximumArgs struct {
	max int
}
type minimumArgs struct {
	min int
}
type miniMaxArgs struct {
	min int
	max int
}

func (self exactArgs) Validate(args []string) bool {
	return len(args) == self.num
}
func (self exactArgs) ToString() string {
	if self.num == 1 {
		return "1 argument"
	}
	return fmt.Sprint(self.num) + " arguments"
}

func (self maximumArgs) Validate(args []string) bool {
	return len(args) <= self.max
}
func (self maximumArgs) ToString() string {
	return "< " + fmt.Sprint(self.max) + " arguments"
}

func (self minimumArgs) Validate(args []string) bool {
	return len(args) >= self.min
}
func (self minimumArgs) ToString() string {
	return "> " + fmt.Sprint(self.min) + " arguments"
}

func (self miniMaxArgs) Validate(args []string) bool {
	return len(args) >= self.min && len(args) <= self.max
}
func (self miniMaxArgs) ToString() string {
	return fmt.Sprint(self.min) + " < x < " + fmt.Sprint(self.max) + "arguments"
}

func ExactArgs(num int) exactArgs {
	return exactArgs{num: num}
}
func MinimumArgs(min int) minimumArgs {
	return minimumArgs{min: min}
}
func MaximumArgs(max int) maximumArgs {
	return maximumArgs{max: max}
}
func MinimaxArgs(min int, max int) miniMaxArgs {
	return miniMaxArgs{min: min, max: max}
}
