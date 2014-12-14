use parser::Parser;

pub use self::UserReply::{
   TakeFile,
   MoveToTrash,
   ShowDiff,
   ShowDiffWith,
   ShowDiffBetween,
   Skip,
   Quit,
   Help
};

pub type FileNum = uint;

#[deriving(Show, PartialEq, Eq)]
pub enum UserReply 
{
   TakeFile(FileNum),
   MoveToTrash,
   ShowDiff,
   ShowDiffWith(FileNum),
   ShowDiffBetween(FileNum, FileNum),
   Skip,
   Quit,
   Help
}

pub fn parse(input: &String, num_conf_files: uint) -> Option<UserReply>
{
   if input.is_empty() || num_conf_files == 0 {
      return None;
   }

   let lowercase_input: String = input.chars()
                                      .map(|c| c.to_lowercase())
                                      .collect();

   let mut parser = Parser::new(lowercase_input.as_slice());
   parser.skip_whitespace();
   match parser.take_char() {
      Err(..) => None,

      Ok(c)   => {
         parser.skip_whitespace();
         let nothing_left = parser.eof();
         let uints = take_uints(&mut parser);
         match c {
            't' => {
               match uints.len() {
                  1 if valid_file_num(uints[0], num_conf_files)
                  => Some(TakeFile(uints[0])),

                  _ => None
               }
            }

            'd' => {
               match uints.len() {
                  0 if num_conf_files == 1
                  => Some(ShowDiff),

                  1 if valid_file_num(uints[0], num_conf_files)
                  => Some(ShowDiffWith(uints[0])),

                  2 if valid_file_num(uints[0], num_conf_files) && valid_file_num(uints[1], num_conf_files)
                  => Some(ShowDiffBetween(uints[0], uints[1])),

                  _ => None
               }
            }

            'm'      if nothing_left => Some(MoveToTrash),
            's'      if nothing_left => Some(Skip),
            'q'      if nothing_left => Some(Quit),
            'h' |'?' if nothing_left => Some(Help),

            _ => None,
         }
      }
   }
}

fn valid_file_num(file_num: uint, num_confs: uint) -> bool
{
   file_num > 0 && file_num <= num_confs
}

fn take_uints(parser: &mut Parser) -> Vec<uint>
{
   let mut uints  = Vec::new();
   while ! parser.eof() {
      parser.skip_whitespace();
      match parser.take_uint() {
         Ok(uint) => uints.push(uint),
         Err(..)  => break
      }
   }

   uints
}

#[test]
fn tests()
{
   test_str("t"       , None);
   test_str("t1"      , Some(TakeFile(1)));
   test_str(" t1"     , Some(TakeFile(1)));
   test_str(" t  1"   , Some(TakeFile(1)));
   test_str(" t  1  " , Some(TakeFile(1)));
   test_str(" t  1  2", None);
   test_str("d12"     , Some(ShowDiffWith(12)));
   test_str("d13"     , None);
   test_str("d"       , None);
   test_str("d1 2"    , Some(ShowDiffBetween(1, 2)));
   test_str("d1    2" , Some(ShowDiffBetween(1, 2)));
   test_str("d  1  2" , Some(ShowDiffBetween(1, 2)));
   test_str("D  1  2" , Some(ShowDiffBetween(1, 2)));
   test_str("  m  "   , Some(MoveToTrash));
   test_str("m  "     , Some(MoveToTrash));
   test_str("M  "     , Some(MoveToTrash));
   test_str("M  1"    , None);
   test_str("s"       , Some(Skip));
   test_str("q"       , Some(Quit));
   test_str("qq"      , None);
   test_str("h"       , Some(Help));
   test_str("?"       , Some(Help));
}

#[cfg(test)]
fn test_str(input: &str, reply: Option<UserReply>)
{
   println!("test: {}", input);
   assert_eq!(parse(&input.to_string(), 12), reply);
}
