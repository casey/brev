# brev

Quick and dirty utility functions.

Rust uses explicit error handling, and in most cases this encourages careful coding. However, sometimes you just want to get in, frob some files, and get out, which is where rush comes in.

Rush provides utility functions that allow you to write simple scripts in rust quickly.

Since they don't return errors, they just panic if something goes wrong. For simple scripts that are run interactively, this is might good enough.

## todo:

- can I write a function that takes either bytes or a string?
- change name to brev, lzy, a, _
- use and re-export tempdir
- use and re-export walkdir
- `isfile(path)` check if path is a regular file
- `isdir(path)` check if path is a directory
- `cwd` get current working directory
- `cd` change current working directory
- `say` print something out
- look through perl for useful stuff
- unwrap or die with message
