use file_conflict::{OrigFileName, Details};
use parser::{Parser, ParseError};

// parses a wuala file conflict enoded in the file name in the form:
//
//    '<base_name> (conflicting version <version> from <host>).<extension>'
//
// e.g: 
//
//    'x_original (conflicting version 5 from blub).txt'
//
pub fn parse(file_name: &str) -> Result<(OrigFileName, Details), ParseError>
{
   let mut parser = Parser::new(file_name);
   let mut base_name = try!(parser.take_while(|c| c != '('));

   // drops the whitespace before the '('
   base_name.pop();

   try!(parser.skip("(conflicting version "));
   let version = try!(parser.take_uint());
   try!(parser.skip(" from "));
   let host = try!(parser.take_while(|c| c != ')'));
   try!(parser.skip(")"));
      
   if ! parser.eof() {
      let extension = try!(parser.take_till_eof());
      base_name.push_str(extension.as_slice());
   }

   let details = format!("Version {} from {}", version, host);
   Ok((base_name, details))
}
