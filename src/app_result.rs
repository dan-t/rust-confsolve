use std::io::IoError;
use std::error::FromError;
use std::fmt::{Show, Formatter, FormatError};

/// The result used in the whole application.
type AppResult<T> = Result<T, AppError>;

/// The generic error used in the whole application.
pub struct AppError
{
   error: String
}

impl AppError
{
   pub fn from_str(str: &str) -> AppError
   {
      AppError { error: str.to_string() }
   }

   pub fn from_string(string: &String) -> AppError
   {
      AppError { error: string.clone() }
   }
}

impl Show for AppError
{
   fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError>
   {
      writeln!(f, "{}", self.error)
   }
}

impl FromError<IoError> for AppError
{
   fn from_error(err: IoError) -> AppError
   {
      AppError { error: format!("{}", err) }
   }
}
