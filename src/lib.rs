extern crate glob;

mod tests;

pub fn glob(pattern: &str) -> Vec<String> {
  match glob::glob(pattern) {
    Ok(matches) => matches
      .filter_map(Result::ok)
      .filter_map(|p| p.as_path().to_str().map(|s| s.to_string()))
      .collect(),
    Err(err) => panic!("glob: error globbing: {}", err),
  }
}

pub fn touch(path: &str) {
  if let Err(err) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
    panic!("touch: {}", err)
  }
}

pub fn mkdir(path: &str) {
  if let Err(err) = std::fs::create_dir(path) {
    panic!("mkdir: {}", err)
  }
}

pub fn rmdir(path: &str) {
  if let Err(err) = std::fs::remove_dir(path) {
    panic!("rmdir: {}", err)
  }
}

pub fn rm(path: &str) {
  if let Err(err) = std::fs::remove_file(path) {
    panic!("rm: {}", err);
  }
}
