package argparse

import (
	"fmt"
	"os"
)


type ParsedArgs struct {

}

type Option struct {
	FullName, AbrevName, HelpMsg string
	TakesArg bool
}

type Argument struct {
	Name, HelpMsg string
}

type Command struct {
	ProgName, Description, HelpMsg string
	Options []Option
	Arguments []Argument
	Subcommands []Command
}

func NewCommand(progName, description string) Command {
	c := Command {
		ProgName: progName,
		Description: description,
	}

	c.Options = append(c.Options, Option {
		FullName: "--help",
		AbrevName: "-h",
		HelpMsg: "Show this error message.",
		TakesArg: false,
	})

	return c
}

func (p *Command) AddOption(opt Option) {
	p.Options = append(p.Options, opt)
}

func (p *Command) AddArgument(arg Argument) {
	p.Arguments = append(p.Arguments, arg)
}

func (p *Command) AddSubcommand(subcmd Command) {
	p.Subcommands = append(p.Subcommands, subcmd)
}

func (c Command) PrintHelp() {
	fmt.Println("Help goes here for command: ", c.ProgName)
}

func (c Command) Parse() ParsedArgs {
	parseFromIndex(0, c)
	return ParsedArgs{}
}

func parseFromIndex(index int, c Command) ParsedArgs {
	args := os.Args[index+1:]
	
	// Get the next subcommand
	for i:=0; i<len(c.Subcommands); i++ {
		if c.Subcommands[i].ProgName == args[0] {
			return parseFromIndex(index+1, c.Subcommands[i])
		}
	}

	for i:=0; i<len(args); i++ {
		for j:=0; j<len(c.Options); j++ {
			if c.Options[j].AbrevName == args[i] || c.Options[j].FullName == args[i] {
				c.PrintHelp()
			}
		}
	}
	return ParsedArgs{}
}
