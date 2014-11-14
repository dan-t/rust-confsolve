use std::fmt::{Show, Formatter, FormatError};

// the file name of the original file,
// without the details of the conflict
pub type OrigFileName = String;

// description of the details of the conflict
pub type Details = String;

// the details and the path of one conflicting file
pub struct ConflictingFile
{
   details:  String,
   path   :  Path
}

// one conflict with all of its conflicting files
pub struct Conflict
{
   original_path    :  Path,
   conflicting_files:  Vec<ConflictingFile>
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
      write!(f, "Conflict (original_path: {}, conflicting_files: {})",
             self.original_path.display(), self.conflicting_files)
   }
}
