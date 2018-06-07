use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct BadContractError(pub i32);

impl fmt::Display for BadContractError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Specified contract is not valid: {}", self.0)
  }
}

impl Error for BadContractError {
  fn description(&self) -> &str {
    "Invalid contract_id"
  }
}
