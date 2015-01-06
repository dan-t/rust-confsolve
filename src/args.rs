use std::os;
use std::path::Path;
use std::fmt::{Show, Formatter, Error};

pub use self::Command::{
   ResolveWuala,
   ResolveDropbox,
   PrintHelp,
   InvalidUsage
};

#[derive(PartialEq)]
pub enum Command
{
   ResolveWuala(Path),
   ResolveDropbox(Path),
   PrintHelp,
   InvalidUsage
}

impl Show for Command
{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
   {
      match *self {
         ResolveWuala(ref path)   => write!(f, "ResolveWuala({})", path.display()),
         ResolveDropbox(ref path) => write!(f, "ResolveDropbox({})", path.display()),
         PrintHelp                => write!(f, "PrintHelp"),
         InvalidUsage             => write!(f, "InvalidUsage")
      }
   }
}

pub fn get_command() -> Command
{
   parse_args(&os::args())
}

pub fn print_help()
{
   println!("
Usage: confsolve wuala <dir>
       confsolve dropbox <dir>
       confsolve --help

Options:
  -h, --help   Show this message.\n");
}

fn is_help_arg(arg: &String) -> bool
{
   *arg == "--help".to_string() || *arg == "-h".to_string()
}

fn is_wuala_arg(arg: &String) -> bool
{
   *arg == "wuala".to_string()
}

fn is_dropbox_arg(arg: &String) -> bool
{
   *arg == "dropbox".to_string()
}

fn parse_args(args: &Vec<String>) -> Command
{
   match args.len() {
      2 if is_help_arg(&args[1])
      => PrintHelp,

      3 if is_wuala_arg(&args[1]) && is_help_arg(&args[2])
      => PrintHelp,

      3 if is_wuala_arg(&args[1])
      => ResolveWuala(Path::new(&args[2])),

      3 if is_dropbox_arg(&args[1]) && is_help_arg(&args[2])
      => PrintHelp,

      3 if is_dropbox_arg(&args[1])
      => ResolveDropbox(Path::new(&args[2])),

      _ => InvalidUsage
   }
}

#[test]
fn tests()
{
   let confsolve = "confsolve".to_string();
   let wuala = "wuala".to_string();
   let dropbox = "dropbox".to_string();
   let help = "--help".to_string();
   let h = "-h".to_string();
   let argh = "argh".to_string();
   let dir = "dir".to_string();
   let dir_path = Path::new("dir");

   assert_eq!(parse_args(&vec![confsolve.clone(), help.clone()]), PrintHelp);
   assert_eq!(parse_args(&vec![confsolve.clone(), wuala.clone(), help.clone()]), PrintHelp);
   assert_eq!(parse_args(&vec![confsolve.clone(), dropbox.clone(), help.clone()]), PrintHelp);
   assert_eq!(parse_args(&vec![confsolve.clone(), wuala.clone(), h.clone()]), PrintHelp);
   assert_eq!(parse_args(&vec![confsolve.clone(), dropbox.clone(), h.clone()]), PrintHelp);
   assert_eq!(parse_args(&vec![confsolve.clone(), wuala.clone()]), InvalidUsage);
   assert_eq!(parse_args(&vec![confsolve.clone(), dropbox.clone()]), InvalidUsage);
   assert_eq!(parse_args(&vec![confsolve.clone(), argh.clone()]), InvalidUsage);
   assert_eq!(parse_args(&vec![confsolve.clone()]), InvalidUsage);
   assert_eq!(parse_args(&vec![confsolve.clone(), argh.clone(), argh.clone()]), InvalidUsage);
   assert_eq!(parse_args(&vec![confsolve.clone(), wuala.clone(), dir.clone()]), ResolveWuala(dir_path.clone()));
   assert_eq!(parse_args(&vec![confsolve.clone(), dropbox.clone(), dir.clone()]), ResolveDropbox(dir_path.clone()));
}
