use std::error::Error;
use std::fmt;
use std::io::Cursor;

use rocket::http::Status;
use rocket::request::Request;
use rocket::response;

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

impl<'r> response::Responder<'r> for BadContractError {
  fn respond_to(self, _: &Request) -> response::Result<'r> {
    response::Response::build()
      .status(Status::NotFound)
      .sized_body(Cursor::new(format!("{}", self)))
      .ok()
  }
}
