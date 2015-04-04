use std::fmt::{Display, Formatter, Error};
use std::path::PathBuf;

pub use self::ConflictType::{
   Wuala,
   Dropbox
};

/// The kind of conflicts to search for and to resolve.
pub enum ConflictType
{
   Wuala,
   Dropbox
}

// the file name of the original file,
// without the details of the conflict
pub type OrigFileName = String;

// description of the details of the conflict
pub type Details = String;

// the details and the path of one conflicting file
#[derive(Clone)]
pub struct ConflictingFile
{
   pub details:  String,
   pub path   :  PathBuf
}

// one conflict with all of its conflicting files
pub struct Conflict
{
   pub original_path    :  PathBuf,
   pub conflicting_files:  Vec<ConflictingFile>
}

impl Display for ConflictingFile
{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
   {
      write!(f, "ConflictingFile (details: {}, path: {})",
             self.details, self.path.display())
   }
}

impl Display for Conflict
{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
   {
      try!(writeln!(f, "Conflicting file: {}", self.original_path.display()));
      for i in 0..self.conflicting_files.len() {
         try!(writeln!(f, "   ({}) {}", i + 1, self.conflicting_files[i].details));
      }

      Ok(())
   }
}
