# esh
esh: The literally unusable shell (or is even a shell) (also my first rust program)

# Fixes to do

Ghost space in 'args'

Merge getoutp with any function 

Make it nearly usable

# Usage

To run a command:

`run ls`

with args:

`run ls with /home/pi`

the args have spaces?

`run ls with '/home/pi/A directory with spaces'`

want to use output? of other commands as arg?

`run ls with $'run echo with /'`

Note: still dont know if quotes will work in quotes (maybe)

Want to change the directory? (Applies to any "environment variable" als wd is the working directory passed to ls (i mean ls only))

`set wd to /home/test`

Want to know what directory you are on? (Applies to any "environment variables")

`get wd`

No you cant use `$'cmd'` in `set` or `get` because my code sucks and ill probably rewrite the entire program in go or some other languages because rust is so frickin hard to work with (also because i am stpid ofc)

# Problems:
```
esh>> run pacman with -Ss n
error: invalid option '-n'

esh>> run whoami
whoami: extra operand ‘’
Try 'whoami --help' for more information.

```

neofetch stops on the "Shell" part