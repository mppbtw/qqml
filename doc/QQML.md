# QQML
The Quiz Question Markup Language is a declarative and highly expressive language for defining revision questions in quite possibly the most overcomplicated and pointless manner possible; perfect for this project :).

## Defining Sections
A single QQML file is known as a section. There is currently no way to source QQML from other files to form larger sections however when using the Yarr QQML manager, the file system is used to give more hierarchical structure to sections and section groups. A QQML file is a list of questions and directives.

## Questions
Questions are initiated with the `ask` keyword followed by a signature and body. The signature of every question type is the same and consists of 3 components:

  1. The question type keyword, e.g. `multichoice`.
  2. The maximum mark for the question enclosed within brackets, e.g. `(2)`.
  3. The question text itself surrounded by quotes. (Note that both double and single quotes are valid in QQML such that a string literal defined by one type of quote can contain the other type).

An example is shown below:
``` javascript
ask multichoice (2) "Who shot ya?"
```

## Multichoice
The actual body for the question depends entirely on the type keyword. The following example shows the syntax of the `multichoice` question type:
``` rust
ask multichoice (2) "Question?" {
  * "Correct answer" (1) -> "Explanation";
  * "Other correct answer" (1) -> "Explanation of this answer";
  * "Incorrect answer";
};
```
This example demonstrates a few important details about multichoice questions, the first of which is how individual answers are defined. Each asterisk began answer only requires the text component, surrounded by quotes (again, single or double). The answer's mark is found in brackets much like in question signatures, however it will default to 0 if omitted. Answer explanations are option pieces of extra information to be shown to the player after they chose their answer (or, if using Yarr's `test` mode, after the section is completed), to add explanations to you answers, use the `->` symbol.

## Hints
In QQML, hints are used in the same way no-matter the type of questions one is writing. The `hints` directive is used to define how many total hints the player is allowed (defaulting to infinity) for the entire section. For each question, the hints are listed after the question body by the `hints` keyword as such:
``` rust
hints 10;

ask <SOME_QUESTION> {
  <QUESTION_BODY>
} hints "first hint", "second hint";
```
