package argparse

type Flag struct {
	Usage   string
	Aliases []string
	Arg     FlagArgumentType
	Long    string
}

type ErrNoSuchFlag struct{}

func (_ ErrNoSuchFlag) Error() string {
	return ""
}

type FlagArgumentType int8

const (
	IntegerFlagArgumentType FlagArgumentType = 0
	UintFlagArgumentType    FlagArgumentType = 1
	StringFlagArgumentType  FlagArgumentType = 2
	NoneFlagArgumentType    FlagArgumentType = 3
)

type AnsweredFlag struct {
	Usage string
	Arg   AnsweredFlagArgument
}

type AnsweredFlagArgument struct {
	argType   FlagArgumentType
	intArg    int
	uintArg   uint
	stringArg string
}

type AnsweredFlags struct {
	Flags []AnsweredFlag
}
func (a *AnsweredFlags) Get(name string) (*AnsweredFlag, error) {
	for _, f := range a.Flags {
		if f.Usage == name {
			return &f, nil
		}

	}
	return nil, ErrNoSuchFlag{}
}
