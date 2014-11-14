use std::iter::Iterator;

pub type FileNum = uint;

#[deriving(Show)]
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

   let num_chars = input.len();
   let mut chars = input.chars()
                        .skip_while(|c| c.is_whitespace())
                        .map(|c| c.to_lowercase());

   match chars.next() {
      Some(c) => {
         match c {
            't' => {
               let uints = parse_uints(&chars.collect::<String>());
               match uints.len() {
                  1 => Some(TakeFile(uints[0])),
                  _ => None
               }
            }

            'd' => {
               let uints = parse_uints(&chars.collect::<String>());
               match uints.len() {
                  0 => Some(ShowDiff),
                  1 => Some(ShowDiffWith(uints[0])),
                  2 => Some(ShowDiffBetween(uints[0], uints[1])),
                  _ => None
               }
            }

            'm'      if num_chars == 1 => Some(MoveToTrash),
            's'      if num_chars == 1 => Some(Skip),
            'q'      if num_chars == 1 => Some(Quit),
            'h' |'?' if num_chars == 1 => Some(Help),

            _ => None,
         }
      }

      None => None,
   }
}

fn parse_uints(string: &String) -> Vec<uint> 
{
   let mut uints  = Vec::new();
   let mut digits = String::new();

   for c in string.chars() {
      if c.is_digit() {
         digits.push(c);
      }
      else if ! digits.is_empty() {
         from_str::<uint>(digits.as_slice()).map(|u| uints.push(u));
         digits.clear();
         continue;
      }
   }

   if ! digits.is_empty() {
      from_str::<uint>(digits.as_slice()).map(|u| uints.push(u));
   }

   uints
}
