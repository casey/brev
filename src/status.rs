use super::signal_from_exit_status;

use super::std::{fmt, io, process};

use std::fmt::Display;

#[derive(Debug)]
pub enum StatusError {
  /// Non-zero exit code
  Code(i32),
  /// IO error upon execution
  Io(io::Error),
  /// Terminated by signal
  Signal(i32),
  /// Unknown failure
  Unknown,
}

impl Display for StatusError {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match *self {
      StatusError::Code(code) => write!(f, "Process exited with status code {}", code),
      StatusError::Io(ref io_error) => write!(f, "Error executing process: {}", io_error),
      StatusError::Signal(signal) => write!(f, "Process terminated by signal {}", signal),
      StatusError::Unknown => write!(f, "Process experienced an unknown failure"),
    }
  }
}

/// Run a command and return the result
pub fn status(mut command: process::Command) -> Result<(), StatusError> {
  match command.status() {
    Ok(status) => {
      if let Some(code) = status.code() {
        if code != 0 {
          Err(StatusError::Code(code))
        } else {
          Ok(())
        }
      } else {
        Err(match signal_from_exit_status(status) {
          Some(signal) => StatusError::Signal(signal),
          None => StatusError::Unknown,
        })
      }
    }
    Err(io_error) => Err(StatusError::Io(io_error)),
  }
}
