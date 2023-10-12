package argparse

type FlagArgumentType int8
const (
    Integer FlagArgumentType = 0
    Uint FlagArgumentType = 1
    String FlagArgumentType = 2
)

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
