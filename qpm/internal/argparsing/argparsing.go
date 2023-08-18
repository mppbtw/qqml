package argparsing

import (
    "os"
)

var current_arg int = 0;

func getArgs() []string {
    return os.Args[1:]
}

func NextArg() string {
    if current_arg == len(getArgs()) {
        return ""
    }
    arg := getArgs()[current_arg]
    current_arg++
    return arg
}

func HasVerbose() bool {
    args := getArgs();
    for i:=0; i<len(args); i++ {
        if args[i] == "-V" || args[i] == "--verbose" {
            return true
        }
    }
    return false
}