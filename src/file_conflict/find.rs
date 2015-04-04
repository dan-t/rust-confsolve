use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::vec::Vec;
use std::path::{Path, PathBuf};

use file_system::walk_files;
use app_result::AppResult;

use super::types::{
   Conflict,
   ConflictingFile,
   ConflictType,
   Wuala,
   Dropbox,
   OrigFileName,
   Details
};

use super::wuala;
use super::dropbox;

/// Finds all conflicts of type `conf_type` in the directory hierarchy starting at `start_dir`.
pub fn find(conf_type: ConflictType, start_dir: &Path) -> AppResult<Vec<Conflict>>
{
   let parse: fn(&str) -> Option<(OrigFileName, Details)> = match conf_type {
      Wuala   => wuala::parse,
      Dropbox => dropbox::parse
   };

   let files = try!(walk_files(start_dir));
   let mut confs_by_orig: HashMap<PathBuf, Vec<ConflictingFile>> = HashMap::new();
   for file in files {
      file.file_name().and_then(|s| s.to_str()).map(|filename| {
         parse(filename).map(|(orig, details)| {
            let mut orig_file = file.clone();
            orig_file.set_file_name(&orig);
            let conf = ConflictingFile {details: details, path: file.clone()};
            match confs_by_orig.entry(orig_file) {
               Occupied(mut entry) => entry.get_mut().push(conf),
               Vacant(entry)       => { entry.insert(vec![conf]); }
            }
         });
      });
   }

   let mut confs = Vec::new();
   for (orig, conf) in confs_by_orig.into_iter() {
      confs.push(Conflict {original_path: orig, conflicting_files: conf});
   }

   Ok(confs)
}
