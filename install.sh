#!/bin/sh

if [ $# -eq 0 ]
then
    echo "No arguments supplied. See https://github.com/MrPiggyPegasus/qqml#installation"
fi

case "$*" in
    (*qqml*)
        echo "Compiling QQML"
        cargo build --release --manifest-path ./Cargo.toml
        echo "Installing QQML"
        sudo mv ./target/release/qqml /usr/local/bin/qqml
        echo "Installation finished, no error reported";; # grub reference????????
    (*qpm*)
        echo "Compiling QPM"
        cd ./qpm
        go build .
        echo "Installing QPM"
        sudo mv ./qpm /usr/local/bin/qpm
        echo "Installation finished, it's all good baby-babey"
esac
