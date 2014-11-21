use super::types::{OrigFileName, Details};
use parser::{Parser, ParseError};

// Parses a wuala file conflict encoded in the file name in the form:
//
//    '<base_name> (conflicting version <version> from <host>).<extension>'
//
// e.g: 
//
//    'x_original (conflicting version 5 from blub).txt'
//
// would return:
//
//    Some("x_original.txt", "Version 5 from blub")
//
pub fn parse(file_name: &str) -> Option<(OrigFileName, Details)>
{
   None
}

fn parse_internal(file_name: &str) -> Result<(OrigFileName, Details), ParseError>
{
   Err("eee".to_string())
}

#[test]
fn tests()
{
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
