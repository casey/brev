use super::{std, signal_from_exit_status};

use super::std::{io, process, fmt};

use std::fmt::Display;

#[derive(Debug)]
pub enum OutputError {
  /// Non-zero exit code
  Code(i32),
  /// IO error upon execution
  Io(io::Error),
  /// Terminated by signal
  Signal(i32),
  /// Unknown failure
  Unknown,
  /// Stdout not UTF-8
  Utf8(std::str::Utf8Error),
}

impl Display for OutputError {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match *self {
      OutputError::Code(code) => write!(f, "Process exited with status code {}", code),
      OutputError::Io(ref io_error) => write!(f, "Error executing process: {}", io_error),
      OutputError::Signal(signal) => write!(f, "Process terminated by signal {}", signal),
      OutputError::Unknown => write!(f, "Process experienced an unknown failure"),
      OutputError::Utf8(ref err) => write!(f, "Could not convert process stdout to UTF-8: {}", err),
    }
  }
}

/// Run a command and return the data it wrote to stdout as a string
pub fn output(mut command: process::Command) -> Result<String, OutputError> {
  match command.output() {
    Ok(output) => {
      if let Some(code) = output.status.code() {
        if code != 0 {
          return Err(OutputError::Code(code));
        }
      } else {
        return Err(match signal_from_exit_status(output.status) {
          Some(signal) => OutputError::Signal(signal),
          None => OutputError::Unknown,
        });
      }
      match std::str::from_utf8(&output.stdout) {
        Err(error) => Err(OutputError::Utf8(error)),
        Ok(utf8) => {
          Ok(if utf8.ends_with('\n') {
            &utf8[0..utf8.len()-1]
          } else if utf8.ends_with("\r\n") {
            &utf8[0..utf8.len()-2]
          } else {
            utf8
          }.to_string())
        }
      }
    }
    Err(io_error) => Err(OutputError::Io(io_error)),
  }
}

