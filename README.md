# brev

Quick and dirty utility functions.

Rust uses explicit error handling, and in most cases this encourages careful coding. However, sometimes you just want to get in, frob some files, and get out, which is where brev comes in.

Brev provides utility functions that allow you to write simple scripts in rust quickly.

Since they don't return errors, they just panic if something goes wrong. For simple scripts that are run interactively, this is might good enough.

And, if you do need to handle errors, you can copy pasta from brev as a starting point.
