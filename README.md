# rus~~t~~h

Quick and dirty utility functions.

Rust uses explicit error handling, and in most cases this encourages careful coding. However, sometimes you just want to get in, frob some files, and get out, which is where rush comes in.

Rush provides utility functions that allow you to write simple scripts in rust quickly.

Since they don't return errors, they just panic if something goes wrong. For simple scripts that are run interactively, this is might good enough.

## todo:

- `slurp(path: &str) -> String` read the contents of a file
- `dump(path: &str, String)` set the contents of a file
