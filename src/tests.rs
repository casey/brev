#![cfg(test)]

extern crate tempdir;

use super::*;

#[test]
fn test_glob() {
  // let tmp = tmpdir::new("brev-test-glob");
  mkdir("tmp");
  touch("tmp/a");
  touch("tmp/b");
  touch("tmp/c");
  assert!(glob("tmp/*").len() == 3);
  rm("tmp/a");
  rm("tmp/b");
  rm("tmp/c");
  rmdir("tmp");
}

#[test]
fn test_dump() {
  mkdir("tmp");
  dump("tmp/foo", "bar");
  assert!(slurp("tmp/foo") == "bar");
  rm("tmp/foo");
  rmdir("tmp");
}

#[test]
fn test_read() {
  mkdir("tmp");
  dump("tmp/foo", vec![1,2,3]);
  assert!(read("tmp/foo") == vec![1,2,3]);
  rm("tmp/foo");
  rmdir("tmp");
}
