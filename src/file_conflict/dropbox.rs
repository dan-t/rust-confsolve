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
      base_name.push_str(extension.as_ref());
   }

   let details = format!("Version {} from {}", version, host);
   Ok((base_name, details))
}

#[test]
fn tests()
{
   test_str("a_original (blub's conflicted copy 2011-04-30)",
            Ok(("a_original".to_string(), "Version 2011-04-30 from blub".to_string())));

   test_str("a_original (blub's conflicted copy 2011-04-30).txt",
            Ok(("a_original.txt".to_string(), "Version 2011-04-30 from blub".to_string())));

   test_str("original (machine's conflicted copy 2011-04-29)",
            Ok(("original".to_string(), "Version 2011-04-29 from machine".to_string())));

   test_str("z_original (laptop's conflicted copy 2011-04-28).qay",
            Ok(("z_original.qay".to_string(), "Version 2011-04-28 from laptop".to_string())));

   test_str("z_original (blub's conflicted copy 2011-04-30)",
            Ok(("z_original".to_string(), "Version 2011-04-30 from blub".to_string())));

   test_str("z_original (laptop's conflicted copy 2011-04-28)",
            Ok(("z_original".to_string(), "Version 2011-04-28 from laptop".to_string())));

   test_str("z_original (machine's conflicted copy 2011-04-29)",
            Ok(("z_original".to_string(), "Version 2011-04-29 from machine".to_string())));

   test_str("z_original", Err("Couldn't skip str '('!".to_string()));
}

#[cfg(test)]
fn test_str(file_name: &str, result: Result<(OrigFileName, Details), ParseError>)
{
   println!("test: {}", file_name);
   assert_eq!(parse_internal(file_name), result);
}
