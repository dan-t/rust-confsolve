#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

#![feature(phase)]

extern crate collections;
extern crate docopt;
extern crate serialize;

#[phase(plugin)]
extern crate docopt_macros;

use std::os::{set_exit_status, getenv};
use std::path::Path;
use std::io;
use std::io::IoResult;
use std::io::IoError;
use std::io::IoErrorKind::{OtherIoError};
use std::io::process::Command;

use std::io::fs::{
   PathExtensions,
   mkdir_recursive,
   unlink,
   copy
};

use file_conflict::{
   find,
   ConflictType,
   Wuala,
   Dropbox,
   Conflict,
   ConflictingFile
};

use user_reply::{
   UserReply,
   TakeFile,
   MoveToTrash,
   ShowDiff,
   ShowDiffWith,
   ShowDiffBetween,
   Skip,
   Quit,
   Help
};

mod file_system;
mod user_reply;
mod wuala_conflict;
mod dropbox_conflict;
mod file_conflict;
mod file_conflict_types;
mod parser;
mod appdirs;
mod args;
mod stream;

fn main() 
{
//   let args = args::get();
//   let conf_type = if args.cmd_wuala { Wuala } else { Dropbox };
//   match resolve_conflicts(conf_type, &Path::new(args.arg_dir)) {
   match resolve_conflicts(Wuala, &Path::new("/home/dan/test/confsolve")) {
      Ok(..)   => {}
      Err(err) => {
         let mut stderr = io::stderr();
         let _ = writeln!(stderr, "confsolve: {}", err);
         set_exit_status(1);
      }
   }
}

/// Finds file conflicts of type `conf_type` starting at the directory `start_dir`,
/// recursively visiting every file, asking the user how each conflict should
/// be handled and then executing the user command.
fn resolve_conflicts(conf_type: ConflictType, start_dir: &Path) -> IoResult<()>
{
   let mut stdin = io::stdin();
   let confs = try!(file_conflict::find(conf_type, start_dir));
   for conf in confs.iter() {
      if ! conf.original_path.is_file() {
         println!("\nFound conflicts for the file '{}', but the file itself is missing! Skipping it.",
                  conf.original_path.display());
         continue;
      }

      println!("\n{}", conf);
      loop {
         print!("{}", "(T)ake File (NUM) | (M)ove to Trash | Show (D)iff (NUM [NUM]) | (S)kip | (Q)uit | (H)elp: ");
         let mut line = try!(stdin.read_line());
         match user_reply::parse(&line) {
            Some(reply) if user_reply::valid(reply, conf.conflicting_files.len()) => {
               match reply {
                  TakeFile(num) => {
                     let ref take_file = conf.conflicting_files[num - 1].path;
                     for conf_file in conf.conflicting_files.iter() {
                        if conf_file.path != *take_file {
                           try!(move_to_trash(&conf_file.path));
                        }
                     }

                     try!(move_to_trash(&conf.original_path));
                     try!(copy(take_file, &conf.original_path));
                  }

                  MoveToTrash => {
                     for conf_file in conf.conflicting_files.iter() {
                        try!(move_to_trash(&conf_file.path));
                     }
                  }

                  ShowDiff => {
                     try!(show_diff(&conf.original_path, &conf.conflicting_files[0].path));
                  }

                  ShowDiffWith(num) => {
                     try!(show_diff(&conf.original_path, &conf.conflicting_files[num - 1].path));
                  }

                  ShowDiffBetween(num1, num2) => {
                     try!(show_diff(&conf.conflicting_files[num1 - 1].path,
                                    &conf.conflicting_files[num2 - 1].path));
                  }

                  Skip => { break; }
                  Quit => { return Ok(()); }
                  Help => try!(print_runtime_help())
               }
            }

            Some(_) | None => {
               // remove newline at end of line
               line.pop();
               println!("\nInvalid user input: '{}' !\n", line);
            }
         }
      }
   }

   Ok(())
}

/// Call the diff command specified by the environment variable `CONFSOLVE_DIFF`
/// or - if not defined - `gvimdiff -f` with the files `file1` and `file2`.
fn show_diff(file1: &Path, file2: &Path) -> IoResult<()>
{
   let diff_cmd_and_args = getenv("CONFSOLVE_DIFF").unwrap_or("gvimdiff -f".to_string());
   let diff_cmd_and_args = diff_cmd_and_args.as_slice().split(' ').collect::<Vec<&str>>();

   let diff_cmd = diff_cmd_and_args[0];

   let mut args = diff_cmd_and_args.iter().skip(1).collect::<Vec<&str>>();
   args.push(file1.as_str().unwrap());
   args.push(file2.as_str().unwrap());

   let mut cmd = Command::new(diff_cmd);
   for arg in args.iter() {
      cmd = cmd.arg(arg);
   }

   try!(cmd.output());
   Ok(())
}

/// Moves `file` into the trash directory of confsolve.
fn move_to_trash(file: &Path) -> IoResult<()>
{
   let filename = 
      try!(file.filename_str()
               .ok_or(IoError { kind: OtherIoError,
                                desc: "Couldn't get filename from path",
                                detail: None }));

   let mut trash_file = try!(trash_dir());
   trash_file.set_filename(filename);

   try!(copy(file, &trash_file));
   try!(unlink(file));

   Ok(())
}

/// Returns the trash directory of confsolve, where all deleted/moved
/// conflicting files are put into.
fn trash_dir() -> IoResult<Path>
{
   let mut dir = 
      try!(appdirs::cache("confsolve")
              .ok_or(IoError { kind: OtherIoError, desc: "Couldn't read cache directory", detail: None }));

   dir.push("trash");
   if ! dir.is_dir() {
      try!(mkdir_recursive(&dir, io::USER_RWX));
   }

   Ok(dir)
}

fn print_runtime_help() -> IoResult<()>
{
   let trash_dir = try!(trash_dir());
   let dir_str   = trash_dir.display();

   println!("
Runtime Options:
   (T)ake File (NUM) => By pressing 't' and a number (e.g 't1'), the conflicting file with the
                        number NUM is used as the new version. A copy of the
                        current file and the other conflicting files is put
                        into the trash directory '{}'.

   (M)ove to Trash   => By pressing 'm', all conflicting files are
                        moved into the trash directory '{}'.

   Show (D)iff (NUM) => By pressing 'd' and a number (e.g 'd1'), the difference between the
                        current file and the conflicting file NUM is shown.
                        If there's only one conflicting file, then only pressing
                        'd' is sufficient.
                        By pressing 'd' and two numbers (e.g 'd1 2'), the difference between
                        the two conflicting files is shown.
                        The diff tool can be specified by the user by setting the environment
                        variable 'CONFSOLVE_DIFF'. The default diff tool is 'gvimdiff -f'.

   (S)kip            => By pressing 's', the current conflict is skipped
                        and the next one is shown.

   (Q)uit            => By pressing 'q', the application is quit.

   (H)elp            => By pressing 'h', this help is printed.
", dir_str, dir_str);

   Ok(())
}
