use std::io;
use std::convert::From;
use std::fmt::{Display, Formatter, Error};

/// The result used in the whole application.
pub type AppResult<T> = Result<T, AppError>;

/// The generic error used in the whole application.
pub struct AppError
{
   error: String
}

impl AppError
{
   pub fn from_string(string: String) -> AppError
   {
      AppError { error: string }
   }
}

impl Display for AppError
{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
   {
      writeln!(f, "{}", self.error)
   }
}

impl From<io::Error> for AppError
{
   fn from(err: io::Error) -> AppError
   {
      AppError { error: format!("{}", err) }
   }
}
