# QQML 

The Quiz Question Markup Language is a declarative and highly expressive
language for defining revision questions in quite possibly the most
overcomplicated and pointless manner possible; perfect for this project :).

## Usage

QQML is used to outline the structure and behaviour of an
interactive TUI which asks the user a questions and allows one of many
types of response. QQML supports multiple choice, numerical input and
longform questions, the latter of which can be marked automatically using
[OpenAI ChatGPT](https://openai.com/chatgpt). Hints and answer
explanations can also be added to your quizzes to enhance not only the
learning experience but also the level of complexity I have to deal with
as the only developer. The full QQML documentation can be found
[here](https://github.com/MrPiggyPegasus/qqml/blob/main/doc/QQML.md). In order
to run your code, use the `qqml` command. For more information, run

```
qqml --help
```

## Installation 

To install the QQML interpreter and it's related tooling, clone this repository
and use the installation script. The QQML interpreter requires Rust/Cargo and CMake.

```
$ git clone https://github.com/MrPiggyPegasus/qqml
$ cd qqml
$ chmod +x ./install.sh
$ ./install.sh qqml
```
