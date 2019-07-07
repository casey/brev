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
  dump(path.clone() + "/foo", vec![1, 2, 3]);
  assert!(read(path.clone() + "/foo") == vec![1, 2, 3]);
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
  assert!(!isdir(&(path.clone() + "/none")));
  assert!(!isfile(&(path.clone() + "/none")));
}

#[test]
#[allow(unreachable_code)]
fn test_macro_expansions() {
  return;
  die!("");
  die!("{} {} {}", 1, 2, 3);
}

#[test]
fn test_empty() {
  let v: Vec<i32> = empty();
  assert!(v.is_empty());
}

#[test]
fn test_output_success() {
  let mut cmd = process::Command::new("printf");
  cmd.arg("hello");
  match output(cmd) {
    Ok(ref string) if string == "hello" => {}
    result => panic!("expected output success but got: {:?}", result),
  }
}

#[test]
fn test_output_code() {
  let mut cmd = process::Command::new("sh");
  cmd.arg("-c").arg("exit 200");
  match output(cmd) {
    Err(OutputError::Code(200)) => {}
    result => panic!("expected code 200 output error but got: {:?}", result),
  }
}

#[test]
fn test_output_io_error() {
  // Please do not create a utility with the name `abazazzle`,
  // as it might cause the following invocation to succeed,
  // and thus this test to fail
  let cmd = process::Command::new("abazazzle");
  match output(cmd) {
    Err(OutputError::Io(_)) => {}
    result => panic!("expected io output error but got: {:?}", result),
  }
}

#[cfg(unix)]
#[test]
fn test_output_utf8_error() {
  use std::ffi::OsString;
  use std::os::unix::ffi::OsStringExt;

  let mut cmd = process::Command::new("printf");
  cmd.arg(OsString::from_vec(b"\xFF".to_vec()));
  match output(cmd) {
    Err(OutputError::Utf8(_)) => {}
    result => panic!("expected utf8 output error but got: {:?}", result),
  }
}

#[test]
fn test_status_success() {
  let mut cmd = process::Command::new("printf");
  cmd.arg("hello");
  if let Err(error) = status(cmd) {
    panic!("expected status success but got: {:?}", error);
  }
}

#[test]
fn test_status_code() {
  let mut cmd = process::Command::new("sh");
  cmd.arg("-c").arg("exit 200");
  match status(cmd) {
    Err(StatusError::Code(200)) => {}
    result => panic!("expected code 200 status error but got: {:?}", result),
  }
}

#[test]
fn test_status_io_error() {
  // Please do not create a utility with the name `abazazzle`,
  // as it might cause the following invocation to succeed,
  // and thus this test to fail
  let cmd = process::Command::new("abazazzle");
  match status(cmd) {
    Err(StatusError::Io(_)) => {}
    result => panic!("expected io status error but got: {:?}", result),
  }
}
