use std::iter::Iterator;
use std::path::{Path, PathBuf};
use std::vec::Vec;
use app_result::{AppResult, AppError};
use appdirs;
use path_ext::PathExt;

use std::fs::{
   read_dir,
   create_dir_all,
   remove_file,
   copy
};

/// Returns an iterator which will recursively walk the files starting at
/// `start_dir`. This will perform iteration in some top-down order. 
/// The contents of unreadable subdirectories are ignored.
pub fn walk_files(start_dir: &Path) -> AppResult<Files> 
{
   match read_dir(start_dir) {
      Ok(contents) => {
         let paths = contents
            .filter_map(|c| c.ok().map(|e| e.path()))
            .collect::<Vec<PathBuf>>();
         
         Ok(Files::new(paths))
      }

      Err(err) => Err(AppError::from_string(format!("{}", err)))
   }
}

/// Moves `file` into the trash directory of confsolve.
pub fn move_to_trash(file: &Path) -> AppResult<()>
{
   let filename = try!(
      file.file_name()
         .ok_or(AppError::from_string(format!("Couldn't get filename from path '{}'!", file.display())))
   );

   let mut trash_file = try!(trash_dir());
   trash_file.push(filename);
   let trash_file = try!(unique_file(&trash_file));

   try!(copy(file, &trash_file));
   try!(remove_file(file));

   Ok(())
}

pub fn move_file(from_file: &Path, to_file: &Path) -> AppResult<()>
{
   try!(copy(from_file, to_file));
   try!(remove_file(from_file));

   Ok(())
}

/// Returns the trash directory of confsolve, where all deleted/moved files are put into.
pub fn trash_dir() -> AppResult<PathBuf>
{
   let mut dir = try!(appdirs::cache("confsolve")
      .ok_or(AppError::from_string(format!("Couldn't get cache directory!"))));

   dir.push("trash");
   if ! dir.is_dir() {
      try!(create_dir_all(&dir));
   }

   Ok(dir)
}

/// Returns a unique path for `file`, by adding a suffix to `file` until it's unique.
pub fn unique_file(file: &Path) -> AppResult<PathBuf>
{
   let mut file_buf = file.to_path_buf();
   if ! file_buf.is_file() {
      return Ok(file_buf);
   }

   let filename_str = try!(file_buf.file_name()
      .and_then(|f| f.to_str())
      .map(|f| f.to_string())
      .ok_or(AppError::from_string(format!("Couldn't get filename_str of '{}'!", file_buf.display())))
   );

   for i in 2..10000 {
      file_buf.set_file_name(&format!("{}-{}", filename_str, i));
      if ! file_buf.is_file() {
         return Ok(file_buf);
      }
   }

   Err(AppError::from_string(format!("Couldn't get a unique path for '{}'!", file_buf.display())))
}

/// An iterator which walks over Files
pub struct Files 
{
   stack: Vec<PathBuf>,
}

impl Files 
{
   fn new(paths: Vec<PathBuf>) -> Files 
   {
      let mut files = Files {stack: Vec::with_capacity(10_000)};
      files.stack.extend(paths.into_iter());
      files
   }
}

impl Iterator for Files
{
   type Item = PathBuf;

   fn next(&mut self) -> Option<PathBuf> 
   {
      loop {
         match self.stack.pop() {
            None => break,

            Some(path) => {
               if path.is_file() { return Some(path); }

               if path.is_dir() {
                  match read_dir(&path) {
                     Err(..) => {}

                     Ok(contents) => {
                        let (size, _) = contents.size_hint();
                        if self.stack.capacity() < self.stack.len() + size {
                           self.stack.reserve(size);
                        }

                        let paths = contents.filter_map(|c| c.ok().map(|e| e.path()));
                        self.stack.extend(paths);
                     }
                  }
               }
            }
         }
      }

      None
   }
}
