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

use std::path::Path;
use std::io;
use std::io::IoResult;
use std::os;

use file_conflict::{
   find,
   ConflictType,
   Wuala,
   Dropbox,
   Conflict,
   ConflictingFile
};

use user_reply::UserReply;

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
         os::set_exit_status(1);
      }
   }
}

fn resolve_conflicts(conf_type: ConflictType, start_dir: &Path) -> IoResult<()>
{
   let confs = try!(file_conflict::find(conf_type, start_dir));
   for c in confs.iter() {
      try!(handle_conflict(c));
   }

   Ok(())
}

fn handle_conflict(conf: &Conflict) -> IoResult<()>
{
//   if ! conf.original_path.is_file() {
//      println!("Found conflicts for '{}', but the file itself is missing! Skipping it.", conf.original_path.display());
//      return Ok(());
//   }

   println!("{}", conf);
   print!("{}", "(T)ake File (NUM) | (M)ove to Trash | Show (D)iff (NUM [NUM]) | (S)kip | (Q)uit | (H)elp: ");
   let mut stdin = io::stdin();
   loop {
      let line = try!(stdin.read_line());
      println!("line: {}", line);
      let opt_reply = user_reply::parse(&line);
      println!("reply: {}", opt_reply);
      match opt_reply {
         Some(reply)
            if user_reply::valid(reply, conf.conflicting_files.len()) => {
               try!(execute_reply(reply));
               break; 
            }

         Some(_) | None => { println!("Invalid user input!"); }
      }
   }

   Ok(())
}

fn execute_reply(reply: UserReply) -> IoResult<()>
{
   Ok(())
}

/// Returns the 'trash' directory of confsolve, where all deleted/moved
/// conflicting files are put into.
fn trash_dir() -> IoResult<Path>
{
   Ok(Path::new(""))
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
