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

type Parser struct {
	ProgName, Description, HelpMsg string
	Options []Option
	Arguments []Argument
}

func NewParser(progName, description string) Parser {
	p := Parser {
		ProgName: progName,
		Description: description,
	}

	p.Options = append(p.Options, Option {
		FullName: "--help",
		AbrevName: "-h",
		HelpMsg: "Show this error message.",
		TakesArg: false,
	})

	return p
}

func (p Parser) AddOption(opt Option) {
	p.Options = append(p.Options, opt)
}

func (p Parser) AddArgument(arg Argument) {
	p.Arguments = append(p.Arguments, arg)
}

func (p Parser) PrintHelp() {
	fmt.Println("Help goes here")
}

func (p Parser) Parse() {
	args := os.Args[1:]
	for i:=0; i<len(args); i++ {
		for j:=0; j<len(p.Options); j++ {
			if p.Options[j].AbrevName == args[i] || p.Options[j].FullName == args[i] {
				p.PrintHelp()
			}
		}
	}
}
