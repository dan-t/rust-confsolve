use super::types::{OrigFileName, Details};
use parser::{Parser, ParseError};

// Parses a dropbox file conflict encoded in the file name in the form:
//
//    `<base_name> (<host>'s conflicted copy <date>).txt`
//
// e.g: 
//
//    `x_original (blub's conflicted copy 2011-04-30).txt`
//
// would return:
//
//    Some("x_original.txt", "Version 2011-04-30 from blub")
//
pub fn parse(file_name: &str) -> Option<(OrigFileName, Details)>
{
   parse_internal(file_name).ok()
}

fn parse_internal(file_name: &str) -> Result<(OrigFileName, Details), ParseError>
{
   let mut parser = Parser::new(file_name);
   let mut base_name = parser.take_while(|c| c != '(');

   // drops the whitespace before the '('
   base_name.pop();

   try!(parser.skip("("));
   let host = parser.take_while(|c| c != '\'');

   try!(parser.skip("'s conflicted copy "));

   let version = parser.take_while(|c| c != ')');
   try!(parser.skip(")"));

   if ! parser.eof() {
      let extension = parser.take_till_eof();
      base_name.push_str(extension.as_slice());
   }

   let details = format!("Version {} from {}", version, host);
   Ok((base_name, details))
}

#[test]
fn tests()
{
   test_str("a_original (blub's conflicted copy 2011-04-30)", "a_original", "Version 2011-04-30 from blub");
   test_str("a_original (blub's conflicted copy 2011-04-30).txt", "a_original.txt", "Version 2011-04-30 from blub");
   test_str("original (machine's conflicted copy 2011-04-29)", "original", "Version 2011-04-29 from machine");
   test_str("z_original (laptop's conflicted copy 2011-04-28).qay", "z_original.qay", "Version 2011-04-28 from laptop");
}

#[cfg(test)]
fn test_str(file_name: &str, orig_name: &str, details: &str)
{
   println!("test: {}", file_name);
   match parse_internal(file_name) {
      Ok((name, det)) => {
         assert_eq!(orig_name.to_string(), name);
         assert_eq!(details.to_string()  , det);
      }

      Err(err) => assert!(false, err)
   }
}
