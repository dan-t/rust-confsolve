use super::types::{OrigFileName, Details};
use parser::{Parser, ParseError};

// Parses a wuala file conflict encoded in the file name in the form:
//
//    `<base_name> (conflicting version <version> from <host>).<extension>`
//
// e.g: 
//
//    `x_original (conflicting version 5 from blub).txt`
//
// would return:
//
//    `Some("x_original.txt", "Version 5 from blub")`
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

   try!(parser.skip("(conflicting version "));
   let version = try!(parser.take_uint());

   let _ = parser.skip(" from ").or(parser.skip(" from"));
   let host = parser.take_while(|c| c != ')');
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
   test_str("original (conflicting version 1 from machine)",
            Ok(("original".to_string(), "Version 1 from machine".to_string())));

   test_str("x_original (conflicting version 5 from blub).txt",
            Ok(("x_original.txt".to_string(), "Version 5 from blub".to_string())));

   test_str("x_original (conflicting version 5 from ).txt",
            Ok(("x_original.txt".to_string(), "Version 5 from ".to_string())));

   test_str("x_original (conflicting version 5 from).txt",
            Ok(("x_original.txt".to_string(), "Version 5 from ".to_string())));

   test_str("x_original (conflicting version 5).txt",
            Ok(("x_original.txt".to_string(), "Version 5 from ".to_string())));

   test_str("z_original", Err("Couldn't skip str '(conflicting version '!".to_string()));
}

#[cfg(test)]
fn test_str(file_name: &str, result: Result<(OrigFileName, Details), ParseError>)
{
   println!("test: {}", file_name);
   assert_eq!(parse_internal(file_name), result);
}
