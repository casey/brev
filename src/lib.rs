extern crate glob;

use std::io::prelude::*;

mod tests;

pub fn glob(pattern: &str) -> Vec<String> {
  match glob::glob(pattern) {
    Ok(matches) => matches
      .filter_map(Result::ok)
      .filter_map(|p| p.as_path().to_str().map(str::to_owned))
      .collect(),
    Err(err) => panic!("glob: error globbing: {}", err),
  }
}

pub fn touch<P: AsRef<std::path::Path>>(path: P) {
  if let Err(err) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
    panic!("touch: {}", err)
  }
}

pub fn mkdir<P: AsRef<std::path::Path>>(path: P) {
  if let Err(err) = std::fs::create_dir(path) {
    panic!("mkdir: {}", err)
  }
}

pub fn rmdir<P: AsRef<std::path::Path>>(path: P) {
  if let Err(err) = std::fs::remove_dir(path) {
    panic!("rmdir: {}", err)
  }
}

pub fn rm<P: AsRef<std::path::Path>>(path: P) {
  if let Err(err) = std::fs::remove_file(path) {
    panic!("rm: {}", err);
  }
}

pub fn slurp<P: AsRef<std::path::Path>>(path: P) -> String {
  let mut file = std::fs::File::open(path).unwrap_or_else(|err| panic!("slurp {}", err));

  let mut s = String::new();

  if let Err(err) = file.read_to_string(&mut s) {
    panic!("slurp: {}", err)
  }

  s
}

pub fn read<P: AsRef<std::path::Path>>(path: P) -> Vec<u8> {
  let mut file = std::fs::File::open(path).unwrap_or_else(|err| panic!("read {}", err));

  let mut v = vec![];

  if let Err(err) = file.read_to_end(&mut v) {
    panic!("read: {}", err)
  }

 v
}

pub fn dump<P: AsRef<std::path::Path>, D: AsRef<[u8]>>(path: P, data: D) {
  let bytes = data.as_ref();
  let count = bytes.len();
  let mut file = std::fs::File::create(path).unwrap_or_else(|err| panic!("dump {}", err));
  match file.write(bytes) {
    Err(err) => panic!("dump: {}", err),
    Ok(n) => if n != count { panic!("dump: only {} of {} bytes written", n, count); }
  }
}
