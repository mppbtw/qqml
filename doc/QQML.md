## Questions

Questions in QQML are defined in 3 parts, the signature, the body and the
metadata. The uestion body's syntax depends on the question type being
used, and so does the metadata. A question's type can be defined in the
signature as seen below:

``` javascript
ask multichoice (1) 'Who created the Rust programming language?'
```

This syntax can be broken up into 3 parts,

* The `ask` keyword
* The question type keyword
* The maximum marks for the question enclosed in brackets
* The actual text of the question itself
