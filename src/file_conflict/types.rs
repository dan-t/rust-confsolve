use std::fmt::{Show, Formatter, Error};
use std::path::Path;

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
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
   {
//      write!(f, "ConflictingFile (details: {}, path: {})",
//             self.details, self.path.display())
      println!("ConflictingFile (details: {}, path: {})",
               self.details, self.path.display())
      Ok(())
   }
}

impl Show for Conflict
{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
   {
//      try!(writeln!(f, "Conflicting file: {}", self.original_path.display()));
//      for i in range(0, self.conflicting_files.len()) {
//         try!(writeln!(f, "   ({}) {}", i + 1, self.conflicting_files[i].details));
//      }

      println!("Conflicting file: {}", self.original_path.display());
      for i in range(0, self.conflicting_files.len()) {
         println!("   ({}) {}", i + 1, self.conflicting_files[i].details);
      }
      Ok(())
   }
}
