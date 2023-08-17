package internal

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
    return getArgs()[current_arg]
}

