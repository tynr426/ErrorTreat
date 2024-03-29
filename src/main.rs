
use std::io;
use std::num;
use std::error;
use std::fmt;
#[derive(Debug)]
enum CliError {
 Io(io::Error),
 Parse(num::ParseIntError),
}
impl fmt::Display for CliError {
 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
 match *self {
 // Both underlying errors already impl `Display`, so we defer to
 // their implementations.
 CliError::Io(ref err) => write!(f, "IO error: {}", err),
 CliError::Parse(ref err) => write!(f, "Parse error: {}", err),
 }
 }
}

impl error::Error for CliError {
 fn description(&self) -> &str {
 // Both underlying errors already impl `Error`, so we defer to their
 // implementations.
 match *self {
 CliError::Io(ref err) => err.description(),
 CliError::Parse(ref err) => err.description(),
 }
 }

 fn cause(&self) -> Option<&error::Error> {
 match *self {
 // N.B. Both of these implicitly cast `err` from their concrete
 // types (either `&io::Error` or `&num::ParseIntError`)
 // to a trait object `&Error`. This works because both error types
 // implement `Error`.
 CliError::Io(ref err) => Some(err),
 CliError::Parse(ref err) => Some(err),
 }
 }
}
fn main() {
    println!("Hello, world!");
}
