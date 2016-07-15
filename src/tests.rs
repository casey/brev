#![cfg(test)]

use super::*;

#[test]
fn test_glob() {
  let (_tmp, path) = tmpdir("test-glob");
  touch(path.clone() + "/a");
  touch(path.clone() + "/b");
  touch(path.clone() + "/c");
  assert!(glob(path.clone() + "/*").len() == 3);
}

#[test]
fn test_dump() {
  let (_tmp, path) = tmpdir("test-glob");
  dump(path.clone() + "/foo", "bar");
  assert!(slurp(path.clone() + "/foo") == "bar");
}

#[test]
fn test_read() {
  let (_tmp, path) = tmpdir("test-glob");
  dump(path.clone() + "/foo", vec![1,2,3]);
  assert!(read(path.clone() + "/foo") == vec![1,2,3]);
}

#[test]
fn test_cd() {
  let (_tmp, path) = tmpdir("test-glob");
  cd(&path);
  cd(cwd());
  let old = cwd();
  assert!(cwd() == old);
  mkdir(old.clone() + "/foo");
  cd(old.clone() + "/foo");
  assert!(cwd() == old + "/foo");
}

#[test]
fn test_is() {
  let (_tmp, path) = tmpdir("test-is");
  touch(path.clone() + "/f");
  assert!(isdir(&path));
  assert!(isfile(&(path.clone() + "/f")));
}

#[test]
#[allow(unreachable_code)]
fn test_macro_expansions() {
  return;
  die!("");
  die!("{} {} {}", 1, 2, 3);
}
