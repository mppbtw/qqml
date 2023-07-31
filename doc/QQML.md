# QQML
The Quiz Question Markup Language is a declarative and highly expressive language for defining revision questions in quite possibly the most overcomplicated and pointless manner possible; perfect for this project :).

## Defining Sections
A single QQML file is known as a section. There is currently no way to source QQML from other files to form larger sections however when using the Yarr QQML manager, the file system is used to give more hierarchical structure to sections and section groups. A QQML file is a list of questions and directives.

## Directives
Directives in QQML are used to specify the way in which the interpreter should run you code. For example, at any point (outside of questions of course) writing:
``` javascript
hints 5;
```
Will give the player 5 hints to use in the entire section. Note that QQML uses semicolons to terminate statements and is not whitespace-sensitive.

## Questions
Questions are initiated with the `ask` keyword followed by a signature and body. The signature of every question type is the same and consists of 3 components:

  1. The question type keyword, e.g. `multichoice`.
  2. The maximum mark for the question enclosed within brackets, e.g. `(2)`.
  3. The question text itself.

An example is shown below:
``` javascript
ask multichoice (2) `Who shot ya?`
```

## Multichoice
The actual body for the question depends entirely on the type keyword.
