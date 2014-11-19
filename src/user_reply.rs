use std::iter::Iterator;
use parser::Parser;

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

pub fn parse(input: &String) -> Option<UserReply> 
{
   if input.is_empty() {
      return None;
   }

   let lowercase_input = input.chars()
                              .map(|c| c.to_lowercase())
                              .collect::<String>();

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
                  1 => Some(TakeFile(uints[0])),
                  _ => None
               }
            }

            'd' => {
               match uints.len() {
                  0 => Some(ShowDiff),
                  1 => Some(ShowDiffWith(uints[0])),
                  2 => Some(ShowDiffBetween(uints[0], uints[1])),
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

pub fn valid(reply: UserReply, num_confs: uint) -> bool
{
   match reply {
      TakeFile(num)
         if ! valid_file_num(num, num_confs)
         => false,

      ShowDiff
         if num_confs != 1
         => false,

      ShowDiffWith(num) 
         if ! valid_file_num(num, num_confs) 
         => false,

      ShowDiffBetween(num1, num2) 
         if ! valid_file_num(num1, num_confs) || ! valid_file_num(num2, num_confs)
         => false,

      _ => true
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
#[cfg(test)]
fn tests()
{
   test_str("t"       , None);
   test_str("t1"      , Some(TakeFile(1)));
   test_str(" t1"     , Some(TakeFile(1)));
   test_str(" t  1"   , Some(TakeFile(1)));
   test_str(" t  1  " , Some(TakeFile(1)));
   test_str(" t  1  2", None);
   test_str("d12"     , Some(ShowDiffWith(12)));
   test_str("d"       , Some(ShowDiff));
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
   assert_eq!(parse(&input.to_string()), reply);
}
