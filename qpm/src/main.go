package main

import (
	"qpm/src/argparse"
)

func main() {
	p := argparse.NewCommand("qpm", "The QQML Package Manager")
	p.AddSubcommand(argparse.NewCommand("run", "Run a quiz"))
	p.Parse()
}
