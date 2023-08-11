## Questions

Questions in QQML are defined in 3 parts, the signature, the body and the
configuration values. The question body's syntax depends on the question
type being used, and so do the conficonfiguration values. A question's
type can be defined in the signature as seen below:

``` javascript
ask multichoice (1) 'Who created the Rust programming language?'
```

This syntax can be broken up into 3 parts,

* The `ask` keyword
* The question type keyword
* The maximum marks for the question enclosed in brackets
* The actual text of the question itself enclosed in quotes(note that both `'`
  and `"` can be used)

## Multichoice Questions

### Body

The above question signature produces a multichoice question, the body
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
