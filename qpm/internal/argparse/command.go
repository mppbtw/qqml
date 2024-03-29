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

import (
	"fmt"
	"os"
	"strings"
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
	Args     ExpectedArgs
}

func (self *Command) displayError(msg string) {
	fmt.Println("An internal error has occurred during the argument parsing process")
	fmt.Println("Please report this to https://gitlab.com/mppbtw/qqml")
	fmt.Println(msg)
	fmt.Println("error occured on the command:", self.Usage)
	fmt.Println("Command debug info:", self)
	os.Exit(1)
}

func (self *Command) init() {
	if len(self.children) == 0 && self.Run == nil {
		self.displayError("This command has no children, but .Run is nil")
	}
	if len(self.flags) > 0 && self.Run == nil {
		self.displayError("Only leaf commands can have consumer-defined flags")
	}

	if self.Args == nil {
		self.Args = ExactArgs(0)
	}

	hasHelp := false

	for i := 0; i < len(self.flags); i++ {
		if self.flags[i].Usage == "--help" {
			hasHelp = true
			break
		}
	}

	if !hasHelp {
		helpFlag := Flag{
			Usage:    "--help",
			Aliases:  []string{},
			Arg:      NoneFlagArgumentType,
			Long:     "Show this help message",
			Required: false,
		}
		aliasIsAvaliable := true
		for i := 0; i < len(self.flags); i++ {
			for j := 0; j < len(self.flags[i].Aliases); j++ {
				if self.flags[i].Aliases[j] == "-h" {
					aliasIsAvaliable = false
					break
				}
			}
		}
		if aliasIsAvaliable {
			helpFlag.Aliases = append(helpFlag.Aliases, "-h")
		}
		self.flags = append(self.flags, helpFlag)
	}
}

func (self *Command) lookupCommand(search string) (*Command, error) {
	for _, cmd := range self.children {
		if cmd.Usage == search {
			return &cmd, nil
		}
	}
	return &Command{}, ErrNoSuchCommand{}
}

func (self *Command) lookupFlag(flag string) (Flag, error) {
	for i := 0; i < len(self.flags); i++ {
		if self.flags[i].Usage == flag {
			return self.flags[i], nil
		}
		for j := 0; j < len(self.flags[i].Aliases); j++ {
			if self.flags[i].Aliases[j] == flag {
				return self.flags[i], nil
			}
		}
	}
	return Flag{}, ErrNoSuchFlag{}
}

func (self *Command) ExecuteLeaf(args []string) {
	flagIndeces := []int{}
	answeredFlags := []AnsweredFlag{}
	i := 0
	for i < len(args) {
		arg := args[i]
		f, err := self.lookupFlag(arg)
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

			if len(args) < i+2 {
				fmt.Println("The flag", f.Usage, "requires an argument of type STRING")
				os.Exit(1)
			}
			flagArg := args[i+1]
			flagIndeces = append(flagIndeces, i+1)
			answeredArg.StringArg = flagArg
			answeredFlag.Arg = answeredArg
			if f.ArgumentCountsForCommand {
				args = append(args, flagArg)
			}
			i++
		}

		answeredFlags = append(answeredFlags, answeredFlag)
		i++
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
	if _, err := flagsResult.Get("--help"); err == nil {
		self.helpScreen()
		os.Exit(0)
	}

	if !self.Args.Validate(args) {
		fmt.Println("Expected", self.Args.ToString() + ", got", len(args))
		os.Exit(1)
	}

	for _, flag := range self.flags {
		if flag.Required {
			if _, err := flagsResult.Get(flag.Usage); err != nil {
				fmt.Println("The required flag", flag.Usage, "was not provided.")
				os.Exit(1)
			}
		}
	}
	self.Run(args, flagsResult)
	os.Exit(0)
}

func (self *Command) helpScreen() {
	fmt.Println(self.Long)
	usageMsg := "Usage:\n "
	usageMsg += self.Usage
	usageMsg += " "

	if len(self.children) != 0 {
		usageMsg += "[command]"
		usageMsg += " "
	}
	if self.Args != ExactArgs(0) {
		usageMsg += "["
		usageMsg += self.Args.ToString()
		usageMsg += "]"
		usageMsg += " "
	}

	if len(self.flags) > 1 {
		usageMsg += "[flags]"
		usageMsg += " "
	}

	fmt.Println("\n" + usageMsg)
	fmt.Println()

	// Commands list
	if len(self.children) != 0 {
		fmt.Println("Commands:")
		lines := [][]string{}
		for _, child := range self.children {
			lines = append(lines, []string{child.Usage, child.Short})
		}
		list := separateLines(lines, 2)
		listLines := strings.Split(list, "\n")
		for i := 0; i < len(listLines); i++ {
			fmt.Println(leftPad(listLines[i], 3))
		}
		fmt.Println()
	}

	// Flags list
	if len(self.flags) != 0 {
		fmt.Println("Flags:")
		lines := [][]string{}
		for _, flag := range self.flags {
			lines = append(lines, []string{flag.Usage, strings.Join(flag.Aliases, " "), flag.Long})
		}
		list := separateLines(lines, 2)
		listLines := strings.Split(list, "\n")
		for _, line := range listLines {
			fmt.Println(leftPad(line, 3))
		}
	}
}

// / Call this on the root command to initiate the parsing sequence
func (self *Command) Execute(args []string) {
	self.init()

	// If it's a leaf command
	if self.Run != nil {
		self.ExecuteLeaf(args)
	}

	if len(args) > 0 {
		arg := args[0]
		cmd, err := self.lookupCommand(arg)
		if err == nil {
			cmd.Execute(args[1:])
		}
		if flag, err := self.lookupFlag(arg); err == nil {
			if flag.Usage == "--help" {
				self.helpScreen()
				os.Exit(0)
			}
		}
		fmt.Println("Unknown argument or subcommand, use --help for more info")
		os.Exit(1)
	} else {
		self.helpScreen()
		os.Exit(1)
	}
	os.Exit(0)
}

func (self *Command) AddCommand(cmd Command) {
	self.children = append(self.children, cmd)
}

func (self *Command) AddFlag(flag Flag) {
	self.flags = append(self.flags, flag)
}
