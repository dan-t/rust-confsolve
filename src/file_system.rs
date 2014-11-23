use std::iter::Iterator;
use std::path::Path;
use std::io::IoResult;
use std::io;
use std::vec::Vec;
use app_result::{AppResult, AppError};
use time;
use appdirs;

use std::io::fs::{
   PathExtensions,
   readdir,
   mkdir_recursive,
   unlink,
   copy
};

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

/// Moves `file` into the trash directory of confsolve.
pub fn move_to_trash(file: &Path) -> AppResult<()>
{
   let filename = try!(file.filename()
      .ok_or(AppError::from_string(format!("Couldn't get filename from path '{}'!", file.display()))));

   let mut trash_file = try!(trash_dir_of_today());
   trash_file.push(filename);

   try!(copy(file, &trash_file));
   try!(unlink(file));

   Ok(())
}

pub fn move_file(from_file: &Path, to_file: &Path) -> AppResult<()>
{
   try!(copy(from_file, to_file));
   try!(unlink(from_file));

   Ok(())
}

/// Returns the trash directory of confsolve runs of today, where all deleted/moved
/// files are put into.
pub fn trash_dir_of_today() -> AppResult<Path>
{
   let time = time::now();
   let day_str = try!(time.strftime("%Y-%m-%d"));

   let mut curr_dir = try!(trash_dir());
   curr_dir.push(format!("{}", day_str));

   if ! curr_dir.is_dir() {
      try!(mkdir_recursive(&curr_dir, io::USER_RWX));
   }

   Ok(curr_dir)
}

/// Returns the trash directory of confsolve, where all deleted/moved files are put into.
pub fn trash_dir() -> AppResult<Path>
{
   let mut dir = try!(appdirs::cache("confsolve")
      .ok_or(AppError::from_string(format!("Couldn't get cache directory!"))));

   dir.push("trash");
   if ! dir.is_dir() {
      try!(mkdir_recursive(&dir, io::USER_RWX));
   }

   Ok(dir)
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
