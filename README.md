## flip ![License](https://img.shields.io/badge/license-BSD--3-ff69b4.png)

Reverse the characters in each line, and print output to `stdout`.
Unicode-aware, so it won't split apart graphemes while reversing.

Reads from `stdin` if no files are given, otherwise concatenates the contents
of each file given on the command line.

### Doesn't `rev` already do that?

`rev` is a basic Unix utility which, in essence, does the same thing as `flip`.
However, `rev` treats its input as if it's ASCII (or at least some variant of
ISO-8859). Unicode is increasingly becoming the norm.

For example, compare the following outputs from `rev` and `flip`:

```
$ cat 'Café' | rev
´efaC

$ cat 'Café' | flip
éfaC
```

Uh oh. `rev` split the diacritic apart, due to it spanning multiple bytes!

Right now `flip` only supports UTF-8 as a character encoding for its input.
In the future, it may support other character encodings.
