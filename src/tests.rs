#![cfg(test)]

use super::*;

#[test]
fn test_glob() {
  mkdir("tmp");
  touch("tmp/a");
  touch("tmp/b");
  touch("tmp/c");
  assert!(glob("tmp/*").len() == 3);
  rmdir("tmp");
}
