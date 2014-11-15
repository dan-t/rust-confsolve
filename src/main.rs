#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

extern crate collections;

use std::path::Path;
use std::io;
use std::os;
use file_system::walk_files;

mod file_system;
mod user_reply;
mod wuala_conflict;
mod file_conflict;
mod parser;

fn main() 
{
   match walk_files(&Path::new("/home/dan/test/confsolve/")) {
      Err(err) => {
         let mut stderr = io::stderr();
         let _ = writeln!(stderr, "confsolve: {}", err);
         os::set_exit_status(1);
      }

      Ok(mut files) => {
         for file in files {
            match wuala_conflict::parse(file.filename_str().unwrap()) {
               Ok((orig_file, details)) => println!("orig: {}, details: {}", orig_file, details),
               Err(..)                  => {}
            }
         }

         println!("END");
      }
   }
}
