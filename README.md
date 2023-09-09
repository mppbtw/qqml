<img align="right" width="30%" src="https://github.com/MrPiggyPegasus/qqml/blob/main/assets/logo.png">

# QQML 
The Quiz Question Markup Language is a declarative and highly expressive
language for defining revision questions in quite possibly the most
overcomplicated and pointless manner possible; perfect to flex on my teacher
for a homework project.

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
$ qqml --help
```

When using this command, use the -j flag to recieve JSON data about the quiz
that was just played, such as the marks which were achieved. If you wish to
build complex systems which use QQML and have features such as saving progress,
this may be useful.

## Installation 

To install the QQML interpreter and it's related tooling, clone this repository
and use the installation script. The QQML interpreter requires Rust/Cargo and CMake.

```
$ git clone https://github.com/MrPiggyPegasus/qqml
$ cd qqml
$ chmod +x ./install.sh
$ ./install.sh qqml
```

# Syntax

> Examples are available in [examples.qqml](https://github.com/MrPiggyPegasus/qqml/blob/main/example.qqml)

## Questions

Questions in QQML are defined in 3 parts, the signature, the body and the
configuration values. The question body's syntax depends on the question
type being used, and so do the configuration values. A question's
type can be defined in the signature as seen below:

``` javascript
ask multichoice (1) 'Who created the Rust programming language?'
```

This syntax can be broken up into 3 parts,

* The `ask` keyword
* The question type keyword
* The maximum marks for the question enclosed in brackets
* The actual text of the question itself enclosed in quotes (note that both `'`
  and `"` can be used)

## Multiple Choice Questions

### Body

The above question signature produces a multiple choice question, the body
syntax of which can be seen below:

```javascript
ask multichoice (1) 'Who created the Rust programming language?' {
    * 'Christopher Wallace';
    * 'Graydon Hoare' (1);
    * 'Ken Wheeler';
};
```

Each possible answer starts with a `*` and is followed by the answer text
in quotes. The mark awarded for the answer (defaulting to 0) is placed in
brackets after that.

It is also possible to add explanations to each of the answers using the
`->` token after the marks as in the following example:

```javascript
ask multichoice (1) 'Who created the Rust programming language?' {
    * 'Christopher Wallace';
    * 'Graydon Hoare' (1) -> 'Graydon Hoare created the Rust language in
      2012.';
    * 'Ken Wheeler' -> 'Counterspace moment';
};
```

### Configuration Values

These can be placed after the body, but before the semicolon which
terminates the question. For the `multichoice` question type keyword, one
such value is permitted. `hints` can be used to give some extra help to
the player using the following syntax:

```javascript
ask multichoice (1) 'Who created the Rust programming language?' {
    * 'Christopher Wallace';
    * 'Graydon Hoare' (1) -> 'Graydon Hoare created the Rust language in
      2006.';
    * 'Ken Wheeler' -> 'Counterspace moment';
} hints 'Christopher Wallace died in 1997', 'Rust was created in 2006';
```

### Hints

In order to access these hints, the user must use one of their `hints`
points. For each quiz, the user's max hints can be defined with the
`hints` directive at the top of the file like this:

``` javascript
hints 3;
```

> **_NOTE_** The hints directive doesnt actually *have* to be at the end of the
> file, but its kinda weird to put it anywhere else. It is impossible however
> to redefine the number of hints later in the program.
