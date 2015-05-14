#![cfg_attr(test, allow(dead_code))]

use std::path::Path;
use std::io::{self, Write};
use std::process::{self, Command};
use std::env;
use path_ext::PathExt;

use file_conflict::{
   ConflictType,
   Wuala,
   Dropbox
};

use user_reply::UserReply::{
   TakeFile,
   MoveToTrash,
   ShowDiff,
   ShowDiffWith,
   ShowDiffBetween,
   Skip,
   Quit,
   Help
};

use app_result::{
   AppResult,
   AppError
};

use args::{
   ResolveWuala,
   ResolveDropbox,
   PrintHelp,
   InvalidUsage
};

use file_system::{
   move_to_trash,
   move_file,
   trash_dir
};

mod app_result;
mod file_system;
mod file_conflict;
mod parser;
mod user_reply;
mod appdirs;
mod args;
mod path_ext;

fn main() 
{
   let cmd = args::get_command();
   match cmd {
      ResolveWuala(path) => {
         resolve_conflicts(Wuala, &path)
            .unwrap_or_else(|err| { exit_with_error(&err); });
      }

      ResolveDropbox(path) => {
         resolve_conflicts(Dropbox, &path)
            .unwrap_or_else(|err| { exit_with_error(&err); });
      }

      PrintHelp => args::print_help(),

      InvalidUsage => {
         args::print_help();
      }
   }
}

fn exit_with_error(err: &AppError)
{
   writeln!(&mut io::stderr(), "{}", err).unwrap();
   process::exit(1);
}

/// Finds file conflicts of type `conf_type` starting at the directory `start_dir`,
/// recursively visiting every file, asking the user how each conflict should
/// be handled and then executing the user command.
fn resolve_conflicts(conf_type: ConflictType, start_dir: &Path) -> AppResult<()>
{
   let mut stdin = io::stdin();
   let mut stdout = io::stdout();

   let confs = try!(file_conflict::find(conf_type, start_dir));
   for conf in confs.iter() {
      if ! conf.original_path.is_file() {
         println!("\nFound conflicts for the file '{}', but the file itself is missing! Skipping it.",
                  conf.original_path.display());
         continue;
      }

      let num_conf_files = conf.conflicting_files.len();
      println!("\n{}", conf);

      loop {
         print!("{}", "(T)ake File (NUM) | (M)ove to Trash | Show (D)iff (NUM [NUM]) | (S)kip | (Q)uit | (H)elp: ");
         let _ = stdout.flush();

         let mut line = String::new();
         try!(stdin.read_line(&mut line));

         match user_reply::parse(&line, num_conf_files) {
            Some(reply) => {
               match reply {
                  TakeFile(num) => {
                     let ref take_file = conf.conflicting_files[num - 1].path;
                     for conf_file in conf.conflicting_files.iter() {
                        if conf_file.path != *take_file {
                           try!(move_to_trash(&conf_file.path));
                        }
                     }

                     try!(move_to_trash(&conf.original_path));
                     try!(move_file(take_file, &conf.original_path));

                     break;
                  }

                  MoveToTrash => {
                     for conf_file in conf.conflicting_files.iter() {
                        try!(move_to_trash(&conf_file.path));
                     }

                     break;
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

            None => {
               // remove newline at end of line
               line.pop();
               println!("\nInvalid user input: '{}' !\n", line);
            }
         }
      }
   }

   Ok(())
}

/// Calls the diff command specified by the environment variable `CONFSOLVE_DIFF`
/// or - if not defined - `gvimdiff -f` with the files `file1` and `file2`.
fn show_diff(file1: &Path, file2: &Path) -> AppResult<()>
{
   let diff_cmd_and_args = env::var("CONFSOLVE_DIFF").unwrap_or("gvimdiff -f".to_string());
   let diff_cmd_and_args = diff_cmd_and_args.split(' ').collect::<Vec<&str>>();

   let diff_cmd = diff_cmd_and_args[0];

   let args = diff_cmd_and_args.iter().skip(1);
   let mut cmd = Command::new(diff_cmd);
   for arg in args {
      cmd.arg(arg);
   }

   cmd.arg(file1);
   cmd.arg(file2);

   try!(cmd.output());
   Ok(())
}

fn print_runtime_help() -> AppResult<()>
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

   (H)elp            => By pressing 'h', this help is printed.\n", dir_str, dir_str);

   Ok(())
}
