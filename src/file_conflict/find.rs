use std::io::IoResult;
use std::io::IoErrorKind::{OtherIoError};
use std::io::IoError;
use std::collections::HashMap;
use std::collections::hash_map::{Entry, Occupied, Vacant};
use std::vec::Vec;

use file_system::walk_files;
use app_result::{AppResult, AppError};

use super::types::{
   OrigFileName,
   Details,
   Conflict,
   ConflictingFile,
   ConflictType,
   Wuala,
   Dropbox
};

use super::wuala;
use super::dropbox;

/// Finds all conflicts of type `conf_type` in the directory hierarchy starting at `start_dir`.
pub fn find(conf_type: ConflictType, start_dir: &Path) -> AppResult<Vec<Conflict>>
{
   let parse = match conf_type {
      Wuala   => wuala::parse,
      Dropbox => dropbox::parse
   };

   let mut files = try!(walk_files(start_dir));
   let mut confs_by_orig: HashMap<Path, Vec<ConflictingFile>> = HashMap::new();
   for file in files {
      let filename = try!(file.filename_str()
         .ok_or(AppError::from_string(format!("Couldn't get filename from path '{}'!", file.display()))));

      parse(filename).map(|(orig, details)| {
         let mut orig_file = file.clone();
         orig_file.set_filename(orig);
         let conf = ConflictingFile {details: details, path: file.clone()};
         match confs_by_orig.entry(orig_file) {
            Occupied(mut entry) => entry.get_mut().push(conf),
            Vacant(entry)       => { entry.set(Vec::from_elem(1, conf)); }
         }
      });
   }

   let mut confs = Vec::new();
   for (orig, conf) in confs_by_orig.into_iter() {
      confs.push(Conflict {original_path: orig, conflicting_files: conf});
   }

   Ok(confs)
}
