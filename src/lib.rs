extern crate glob;
extern crate tempdir;

use std::io::prelude::*;

pub fn tmpdir<S: AsRef<str>>(prefix: S) -> (tempdir::TempDir, String) {
  let tmp = tempdir::TempDir::new(prefix.as_ref()).unwrap_or_else(|err| panic!("tmpdir: failed to create temporary directory: {}", err));
  let path = tmp.path().to_str().unwrap_or_else(|| panic!("tmpdir: path was not valid UTF-8")).to_owned();
  return (tmp, path);
}

pub fn glob<P: AsRef<str>>(pattern: P) -> Vec<String> {
  match glob::glob(pattern.as_ref()) {
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

pub fn cd<P: AsRef<std::path::Path>>(path: P) {
  if let Err(err) = std::env::set_current_dir(path) {
    panic!("cd: {}", err)
  }
}

pub fn cwd() -> String {
  match std::env::current_dir() {
    Ok(pathbuf) => pathbuf.to_str().unwrap_or_else(|| panic!("cwd: cwd was not a valid UTF-8 string")).to_string(),
    Err(err) => panic!("cwd: {}", err),
  }
}

pub fn can(command: &str) -> bool {
  let paths = match std::env::var_os("PATH") {
    Some(os_paths) => os_paths.to_str().unwrap_or_else(|| panic!("can: PATH environment variable is not valid UTF-8")).to_owned(),
    None => panic!("can: PATH environment variable is not set"),
  };

  for path in paths.split(":") {
    let candidate = format!("{}/{}", path, command);
    if isfile(&candidate) {
      return true;
    }
  }
  false
}

pub fn isfile<P: AsRef<std::path::Path>>(path: P) -> bool {
  match std::fs::metadata(path) {
    Ok(metadata) => metadata.is_file(),
    Err(err) => panic!("isfile: could not retrieve metadata: {}", err),
  }
}

pub fn isdir<P: AsRef<std::path::Path>>(path: P) -> bool {
  match std::fs::metadata(path) {
    Ok(metadata) => metadata.is_dir(),
    Err(err) => panic!("isfile: could not retrieve metadata: {}", err),
  }
}

pub fn say<D: std::fmt::Display> (d: D) {
  println!("{}", d)
}

pub fn warn<D: std::fmt::Display>(d: D) {
  if let Err(err) = writeln!(&mut std::io::stderr(), "{}", d) {
    panic!("warn: {}", err);
  }
}

pub fn die<D: std::fmt::Display>(d: D) -> ! {
  warn(d);
  std::process::exit(-1)
}

#[macro_export]
macro_rules! warn {
  ($($arg:tt)*) => {{
    extern crate std;
    use std::io::prelude::*;
    if let Err(_) = writeln!(&mut std::io::stderr(), $($arg)*) {
      std::process::exit(-1);
    };
  }};
}

#[macro_export]
macro_rules! die {
  ($($arg:tt)*) => {{
    extern crate std;
    warn!($($arg)*);
    std::process::exit(-1)
  }};
}


mod tests;
