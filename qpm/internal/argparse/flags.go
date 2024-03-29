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

type Flag struct {
	Usage    string
	Aliases  []string
	Arg      FlagArgumentType
	Long     string
	Required bool
	// If true, any arguments captured for this flag will count towards the total arg count for the command.
	// Please be very careful not to mistake the flag argument for a command argument in your run function.
	ArgumentCountsForCommand bool
}

type ErrNoSuchFlag struct{}

func (_ ErrNoSuchFlag) Error() string {
	return "There is no such flag"
}

type FlagArgumentType int8

const (
	IntegerFlagArgumentType FlagArgumentType = iota
	UintFlagArgumentType
	StringFlagArgumentType
	NoneFlagArgumentType
)

type AnsweredFlag struct {
	Usage string
	Arg   AnsweredFlagArgument
}

type AnsweredFlagArgument struct {
	ArgType   FlagArgumentType
	IntArg    int
	UintArg   uint
	StringArg string
}

type AnsweredFlags struct {
	Flags []AnsweredFlag
}

func (self *AnsweredFlags) Get(name string) (*AnsweredFlag, error) {
	for _, f := range self.Flags {
		if f.Usage == name {
			return &f, nil
		}

	}
	return nil, ErrNoSuchFlag{}
}
