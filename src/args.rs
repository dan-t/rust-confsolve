use docopt;
use docopt::Docopt;

docopt!(Args deriving Show, "
Usage: confsolve wuala <dir>
       confsolve dropbox <dir>
       confsolve --help

Options:
  -h, --help   Show this message.
")

pub fn get() -> Args
{
   Args::docopt().decode().unwrap_or_else(|e| e.exit())
}
