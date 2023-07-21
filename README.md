# yarr
"Yet Another Revision Resource" - The pointless and overkill homework project

# QQML
The Quiz Question Markup Language is a declarative and highly expressive language for defining revision questions in quite
possibly the most overcomplicated and pointless manner possible; perfect for this project :).

### Defining Question Signatures
Questions are defined in a similar manner to functions in many languages, in that they have a signature which precedes the body
(for example the possible answers to a multiple choice question). It is worth noting that the separation of signature and
implementation is not as important as it is in most general purpose functional programming languages as signatures are not
prototypes and the question body is not the implementation in QQML. The question signature is started with the `ask` keyword and
consists of 3 major components:
* (1) The question type keyword states what sort of question is being asks and therefore affects the syntax of the question's body such as `multichoice` or `calculation`
* (2) The maximum number of marks for the question is shown in brackets and must be an integer value
* (3)The text to show to the player.

## Multiple Questions
``` qqml
ask multichoice (2) "What is the best language?" {
   * "rust" (1);
   * "python" (1);
   * "C++";
}
```
This example defines a multiple choice question worth a maximum of 2 marks. When writing multichoice questions, the body must
consist of asterisk begun and semicolon terminated possible answers. The answers each contain a string literaland the number of
marks each one awards. If no marks are specified the number is assumed to be 0. The order of the answers is the same order in
which they will be shown to the player.

When the user chooses an option, it will be selected but not confirmed until all of the answers are selected.

``` qqml
ask multichoice (1) "Which is the better operating system?" {
  * "Arch Linux" (1) -> "Arch Linux is the best operating system as it gives the user more control over their computing";
  * "Microsoft Windows" -> "Windows is not the best operating system as it gives the user less freedom";
  * "Mac OS";
  * "Gentoo Linux" (1);
}
```
The above example displays the feature of answer explanations which are shown to the player after selecting their answer. If the
user selects the incorrect answer, the explanation (if it exists) of their wrong choice is shown in red, and that of the correct
answer (again, only if it exists) is shown in green along. Explanations are defined using a right arrow after defining the rest
of the choice. Explanations are shown after the test is finished when running in test mode.

This example also shows the slightly different behaviour when the maximum mark is 1. If this is the case then the first answer
the user chooses will be selected and marked.

## Calculation Questions
```qqml
ask calculation (2) "What is 2+2" {
  x = [4, 22] ();
  x = [69, 420] (1);
}
```
This example shows the syntax for the body of calculation questions. each line is an expression which, if true, awards the mark
given in brackets. Square brackets can contain multiple, comma separated values which allow the expression to be true if any of
the values are matched. 
