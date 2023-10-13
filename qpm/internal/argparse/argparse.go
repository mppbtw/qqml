package argparse

import (
	"fmt"
	"os"
	"strings"
)

type FlagArgumentType int8
const (
    IntegerFlagArgumentType FlagArgumentType = 0
    UintFlagArgumentType FlagArgumentType = 1
    StringFlagArgumentType FlagArgumentType = 2
    NoneFlagArgumentType FlagArgumentType = 3
)

type CommandBuilder struct {
    Usage string
    Long string
    Short string
    Run func([]string)
    Args uint
    Flags []Flag
}

type Command struct {
    children []Command
    usage string
    long string
    short string
    flags []Flag
    run func([]string)
}

type Flag struct{
    Usage string
    Aliases []string
    Arg FlagArgumentType
    Long string
}

func NewCommand(cmdInfo CommandBuilder) Command {
    out := Command{
        usage: cmdInfo.Usage,
        long: cmdInfo.Long,
        short: cmdInfo.Short,
        children: []Command{},
        flags: cmdInfo.Flags,
        run: cmdInfo.Run,
    }
    if len(out.flags)>0 && out.run != nil {
        fmt.Println("INTERNAL ERROR: Only leaf commands (without subcommands) can have custom flags!")
        os.Exit(1)
    }

    hasHelp := false

    for i:=0; i<len(out.flags); i++ {
        if out.flags[i].Usage == "--help" {
            hasHelp = true
            break
        }
    }

    if !hasHelp {
        helpFlag := Flag{
            Usage: "--help",
            Aliases: []string{},
            Arg: NoneFlagArgumentType,
            Long: "Show this help message",
        }
        aliasIsAvaliable := true
        for i:=0; i<len(out.flags); i++ {
            for j:=0; j<len(out.flags[i].Aliases); j++ {
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
