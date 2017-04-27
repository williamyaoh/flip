# NAME

flip - Unicode-aware line-reversal utility

# SYNOPSIS

flip [<file> ...]
flip (--help | -h)
flip (--version | -v)

# DESCRIPTION

**flip** reverses the characters in each line it's given. If it's called with
no arguments, it reads from stdin. Otherwise, it concatenates the contents of
each file passed on the command line before reversing the contents of each
line.

**flip** is Unicode-aware and makes sure not to break apart graphemes. For example,
the 'character' "é" actually consists of two *different* Unicode code points:
"e" and " ́". Naively reversing this bytewise would result in " ́e": not likely
what you want. The "e" and its diacritic should intuitively stay together when
the text they're part of gets reversed.

# ENCODINGS

**flip** only takes UTF-8 as input/output. If you need to work with other
character encodings, use **iconv** as a filter.

# SEE ALSO

**rev**(1) 
Same purpose, but not Unicode-aware, and will split apart things that
shouldn't be split apart.

**iconv**(1)
Utility for converting to/from various character encodings.
