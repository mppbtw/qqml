package argparse

type ExpectedArgs interface {
	Validate([]string) bool
}
type exactArgs struct{
	num int
}
type maximumArgs struct{
	max int
}
type minimumArgs struct{
	min int
}
type miniMaxArgs struct{
	min int
	max int
}

func (self exactArgs) Validate(args []string) bool {
	return len(args) == self.num
}
func (self maximumArgs) Validate(args []string) bool {
	return len(args) < self.max
}
func (self minimumArgs) Validate(args []string) bool {
	return len(args) > self.min
}
func (self miniMaxArgs) Validate(args []string) bool {
	return len(args) > self.min && len(args) < self.max
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
