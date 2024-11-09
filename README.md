# Spell check
Simple cli spell checker with a badly implemented attempt at a bloom filter.


The words list is expected to be a text file called `words.txt` containing newline seperated valid words and should be stored in the root directory when compiling.

```
Usage: spell-check [SENTENCE]

Arguments:
  [SENTENCE]  [default: -]

Options:
  -h, --help     Print help
  -V, --version  Print version
```
