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

package argparse

import (
	"fmt"
	"os"
)

type CommandBuilder struct {
	Usage string
	Long  string
	Short string
	Run   func([]string)
	Args  uint
	Flags []Flag
}

type command struct {
	children []command
	usage    string
	long     string
	short    string
	flags    []Flag
	run      func([]string)
}

func NewCommand(cmdInfo CommandBuilder) command {
	out := command{
		usage:    cmdInfo.Usage,
		long:     cmdInfo.Long,
		short:    cmdInfo.Short,
		children: []command{},
		flags:    cmdInfo.Flags,
		run:      cmdInfo.Run,
	}
	if len(out.flags) > 0 && out.run != nil {
		fmt.Println("INTERNAL ERROR: Only leaf commands (without subcommands) can have custom flags!")
		os.Exit(1)
	}

	hasHelp := false

	for i := 0; i < len(out.flags); i++ {
		if out.flags[i].Usage == "--help" {
			hasHelp = true
			break
		}
	}

	if !hasHelp {
		helpFlag := Flag{
			Usage:   "--help",
			Aliases: []string{},
			Arg:     NoneFlagArgumentType,
			Long:    "Show this help message",
		}
		aliasIsAvaliable := true
		for i := 0; i < len(out.flags); i++ {
			for j := 0; j < len(out.flags[i].Aliases); j++ {
				if out.flags[i].Aliases[j] == "-h" {
					aliasIsAvaliable = false
					break
				}
			}
		}
		if aliasIsAvaliable {
			helpFlag.Aliases = append(helpFlag.Aliases, "-h")
		}
		out.flags = append(out.flags, helpFlag)
	}
	return out
}

func (c *command) lookupFlag(flag string) (Flag, error) {
	for i := 0; i < len(c.flags); i++ {
		if c.flags[i].Usage == flag {
			return c.flags[i], nil
		}
		for j := 0; j < len(c.flags[i].Aliases); j++ {
			if c.flags[i].Aliases[j] == flag {
				return c.flags[i], nil
			}
		}
	}
	return Flag{}, ErrNoSuchFlag{}
}

func (c *command) ExecuteLeaf(args []string) {
	flagIndeces := []int{}
	answeredFlags := []AnsweredFlag{}
	i := 0
	for i < len(args) {
		arg := args[i]
		f, err := c.lookupFlag(arg)
		if err != nil {
			i++
			continue
		}
		flagIndeces = append(flagIndeces, i)

		answeredFlag := AnsweredFlag{
			Usage: f.Usage,
		}

		if f.Arg != NoneFlagArgumentType {
			answeredArg := AnsweredFlagArgument{}
			answeredArg.argType = f.Arg

			if f.Arg != StringFlagArgumentType {
				fmt.Println("INTERNAL ERROR: Only the string argument type is implemented yet.")
				os.Exit(1)
			}

			if len(args) < i+1 {
				fmt.Println("The flag", f.Usage, "requires an argument of type STRING")
				os.Exit(1)
			}
			flagArg := args[i+1]
			flagIndeces = append(flagIndeces, i+1)
			i++
			answeredArg.stringArg = flagArg
		}

		answeredFlags = append(answeredFlags, answeredFlag)
	}

	// Sort backwards so we dont have to shift the indeces when removing them from
	// the args
	flagIndeces = backwardsSort(flagIndeces)
	for _, i := range flagIndeces {
		args = remove(args, i)
	}

	flagsResult := AnsweredFlags {
		Flags: answeredFlags,
	}
	if _, err := flagsResult.Get("--help"); err != nil {
		c.helpScreen()
	}
}

func (c *command) helpScreen() {
	fmt.Println("placeholder help screen")
}

// / Call this on the root command to initiate the parsing sequence
func (c *command) Execute(args []string) {

	// If it's a leaf command
	if c.run != nil {
		c.ExecuteLeaf(args)
	}
}
