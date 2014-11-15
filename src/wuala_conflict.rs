use file_conflict::{OrigFileName, Details};
use parser::{Parser, ParseError};

// parses a wuala file conflict encoded in the file name in the form:
//
//    '<base_name> (conflicting version <version> from <host>).<extension>'
//
// e.g: 
//
//    'x_original (conflicting version 5 from blub).txt'
//
// would return:
//
//    Ok("x_original.txt", "Version 5 from blub")
//
pub fn parse(file_name: &str) -> Result<(OrigFileName, Details), ParseError>
{
   let mut parser = Parser::new(file_name);
   let mut base_name = try!(parser.take_while(|c| c != '('));

   // drops the whitespace before the '('
   base_name.pop();

   try!(parser.skip("(conflicting version "));
   let version = try!(parser.take_uint());

   try!(parser.skip(" from ")
              .or(parser.skip(" from"))
              .or(Ok(())));

   let host = try!(parser.take_while(|c| c != ')'));
   try!(parser.skip(")"));
      
   if ! parser.eof() {
      let extension = try!(parser.take_till_eof());
      base_name.push_str(extension.as_slice());
   }

   let details = format!("Version {} from {}", version, host);
   Ok((base_name, details))
}

#[test]
#[cfg(test)]
fn tests()
{
   test_str("original (conflicting version 1 from machine)", "original", "Version 1 from machine");
   test_str("x_original (conflicting version 5 from blub).txt", "x_original.txt", "Version 5 from blub");
   test_str("x_original (conflicting version 5 from ).txt", "x_original.txt", "Version 5 from ");
   test_str("x_original (conflicting version 5 from).txt", "x_original.txt", "Version 5 from ");
   test_str("x_original (conflicting version 5).txt", "x_original.txt", "Version 5 from ");
}

#[cfg(test)]
fn test_str(file_name: &str, orig_name: &str, details: &str)
{
   println!("test: {}", file_name);
   match parse(file_name) {
      Ok((name, det)) => {
         assert_eq!(orig_name.to_string(), name);
         assert_eq!(details.to_string()  , det);
      }

      Err(err) => assert!(false, err)
   }
}
