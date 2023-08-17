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
        sudo mv ./target/release/qqml /usr/local/bin/
        echo "Installation finished, no error reported";; # grub reference????????
esac
