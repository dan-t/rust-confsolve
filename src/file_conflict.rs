use std::io::IoResult;
use std::collections::HashMap;
use std::collections::hash_map::{Entry, Occupied, Vacant};
use std::vec::Vec;

use file_conflict_types::{
   OrigFileName,
   Details
};

pub use file_conflict_types::{
   Conflict,
   ConflictingFile
};

use file_system::walk_files;
use wuala_conflict;
use dropbox_conflict;

pub enum ConflictType
{
   Wuala,
   Dropbox
}

pub fn find(conf_type: ConflictType, start_dir: &Path) -> IoResult<Vec<Conflict>>
{
   let parse = match conf_type {
      Wuala   => wuala_conflict::parse,
      Dropbox => dropbox_conflict::parse
   };

   let mut files = try!(walk_files(start_dir));
   let mut confs_by_orig: HashMap<Path, Vec<ConflictingFile>> = HashMap::new();
   for file in files {
      match parse(file.filename_str().unwrap()) {
         Err(..) => {}

         Ok((orig, details)) => {
            let mut orig_file = file.clone();
            orig_file.set_filename(orig);
            let conf = ConflictingFile {details: details, path: file.clone()};
            match confs_by_orig.entry(orig_file) {
               Occupied(mut entry) => entry.get_mut().push(conf),
               Vacant(entry)       => { entry.set(Vec::from_elem(1, conf)); }
            }
         }
      }
   }

   let mut confs = Vec::new();
   for (orig, conf) in confs_by_orig.into_iter() {
      confs.push(Conflict {original_path: orig, conflicting_files: conf});
   }

   Ok(confs)
}
