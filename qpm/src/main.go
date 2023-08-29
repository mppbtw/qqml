package main

import (
	"qpm/src/argparse"
)

func main() {
	p := argparse.NewParser("qpm", "The QQML Package Manager")
	p.Parse()
}
