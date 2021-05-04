use std::process::Command;
use std::io;
use std::convert::From;
use std::string::FromUtf8Error;
use derive_more::Display;

#[derive(Debug, Display)]
pub enum CLIError {
  #[display(fmt = "Could not spawn process: {}", _0)]
  IOError(String),
  #[display(fmt = "Error running command: {}", _0)]
  CommandError(String),
  #[display(fmt = "Could not decode CLI output")]
  DecodeError
}

impl std::error::Error for CLIError {}

impl From<io::Error> for CLIError {
  fn from(err: io::Error) -> Self {
    CLIError::IOError(err.to_string())
  }
}

impl From<FromUtf8Error> for CLIError {
  fn from(_: FromUtf8Error) -> Self {
    CLIError::DecodeError
  } 
}

pub fn cli_kill(pid: usize) -> Result<String, CLIError> {
  let output = Command::new("cmd")
    .args(&["/C", "taskkill", "/f", "/pid", &pid.to_string()])
    .output()?;

  let cli_output = String::from_utf8(output.stdout)?;

  if output.status.success() {
    Ok(cli_output)
  } else {
    let err_output = String::from_utf8(output.stderr)?;
    Err(CLIError::CommandError(
      format!(
        "Process returned non 0 exit value\n{}\n{}", 
        cli_output, 
        err_output
      )
    ))
  }
}