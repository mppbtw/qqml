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

type ErrNoSuchCommand struct{}

func (e ErrNoSuchCommand) Error() string {
	return "There is no such command"
}

type Command struct {
	children []Command
	Usage    string
	Long     string
	Short    string
	flags    []Flag
	Run      func([]string, AnsweredFlags)
	Args     int
}

func (c *Command) Init() {
	if len(c.flags) > 0 && c.Run != nil {
		fmt.Println("INTERNAL ERROR: Only leaf commands (withc subcommands) can have custom flags!")
		os.Exit(1)
	}

	hasHelp := false

	for i := 0; i < len(c.flags); i++ {
		if c.flags[i].Usage == "--help" {
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
		for i := 0; i < len(c.flags); i++ {
			for j := 0; j < len(c.flags[i].Aliases); j++ {
				if c.flags[i].Aliases[j] == "-h" {
					aliasIsAvaliable = false
					break
				}
			}
		}
		if aliasIsAvaliable {
			helpFlag.Aliases = append(helpFlag.Aliases, "-h")
		}
		c.flags = append(c.flags, helpFlag)
	}
}

func (c *Command) lookupCommand(search string) (*Command, error) {
	for _, cmd := range c.children {
		if cmd.Usage == search {
			return &cmd, nil
		}
	}
	return &Command{}, ErrNoSuchCommand{}
}

func (c *Command) lookupFlag(flag string) (Flag, error) {
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

func (c *Command) ExecuteLeaf(args []string) {
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
			answeredArg.ArgType = f.Arg

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
			answeredArg.StringArg = flagArg
		}

		answeredFlags = append(answeredFlags, answeredFlag)
	}

	// Sort backwards so we dont have to shift the indeces when removing them from
	// the args
	flagIndeces = backwardsSort(flagIndeces)
	for _, i := range flagIndeces {
		args = remove(args, i)
	}

	flagsResult := AnsweredFlags{
		Flags: answeredFlags,
	}
	if _, err := flagsResult.Get("--help"); err != nil {
		c.helpScreen()
	}

	if len(args) != c.Args {
		fmt.Println("Expected", c.Args, "arguments, got", len(args))
		os.Exit(1)
	}

	c.Run(args, flagsResult)
	os.Exit(0)
}

func (c *Command) helpScreen() {
	fmt.Println("placeholder help screen")
}

// / Call this on the root command to initiate the parsing sequence
func (c *Command) Execute(args []string) {

	// If it's a leaf command
	if c.Run != nil {
		c.ExecuteLeaf(args)
	}
}

func (c *Command) AddCommand(cmd Command) {
	c.children = append(c.children, cmd)
}

func (c *Command) AddFlag(flag Flag) {
	c.flags = append(c.flags, flag)
}
