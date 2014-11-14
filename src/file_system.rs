use std::iter::Iterator;
use std::path::Path;
use std::io::IoResult;
use std::io::fs::{PathExtensions, readdir};
use std::vec::Vec;

/// Returns an iterator which will recursively walk the files starting at
/// `start_dir`. This will perform iteration in some top-down order. 
/// The contents of unreadable subdirectories are ignored.
pub fn walk_files(start_dir: &Path) -> IoResult<Files> 
{
   match readdir(start_dir) {
      Ok(contents) => Ok(Files::new(contents)),

      Err(mut err) => {
         err.desc   = "Couldn't read directory";
         err.detail = Some(format!("path='{}'", start_dir.display()));
         Err(err)
      }
   }
}

/// An iterator which walks over Files
pub struct Files 
{
   stack: Vec<Path>,
}

impl Files 
{
   fn new(paths: Vec<Path>) -> Files 
   {
      let mut files = Files {stack: Vec::with_capacity(10_000)};
      files.stack.extend(paths.into_iter());
      files
   }
}

impl Iterator<Path> for Files 
{
   fn next(&mut self) -> Option<Path> 
   {
      loop {
         match self.stack.pop() {
            None => break,

            Some(path) => {
               if path.is_file() { return Some(path); }

               if path.is_dir() {
                  match readdir(&path) {
                     Err(..) => {}

                     Ok(contents) => {
                        if self.stack.capacity() < self.stack.len() + contents.len() {
                           self.stack.reserve(contents.len());
                        }

                        self.stack.extend(contents.into_iter()); 
                     }
                  }
               }
            }
         }
      }

      None
   }
}
