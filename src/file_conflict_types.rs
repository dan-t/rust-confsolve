use std::fmt::{Show, Formatter, FormatError};
use std::path::Path;

// the file name of the original file,
// without the details of the conflict
pub type OrigFileName = String;

// description of the details of the conflict
pub type Details = String;

// the details and the path of one conflicting file
#[deriving(Clone)]
pub struct ConflictingFile
{
   pub details:  String,
   pub path   :  Path
}

// one conflict with all of its conflicting files
pub struct Conflict
{
   pub original_path    :  Path,
   pub conflicting_files:  Vec<ConflictingFile>
}

impl Show for ConflictingFile
{
   fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError>
   {
      write!(f, "ConflictingFile (details: {}, path: {})",
             self.details, self.path.display())
   }
}

impl Show for Conflict
{
   fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError>
   {
      try!(writeln!(f, "Conflicting file: {}", self.original_path.display()));
      for i in range(0, self.conflicting_files.len()) {
         try!(writeln!(f, "   ({}) {}", i + 1, self.conflicting_files[i].details));
      }

      Ok(())
   }
}