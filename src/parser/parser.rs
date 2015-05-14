use super::stream::Stream;

pub struct Parser<'a>
{
   strm: Stream<'a>
}

pub type ParseError = String;

impl<'a> Parser<'a>
{
   pub fn new(input: &str) -> Parser
   {
      Parser {strm: Stream::new(input)}
   }

   pub fn eof(&self) -> bool { self.strm.eof() }

   pub fn skip(&mut self, str: &str) -> Result<(), ParseError>
   {
      if str.is_empty() {
         return Err("Couldn't skip empty str!".to_string());
      }

      if self.eof() {
         return Err(format!("Couldn't skip str '{}'!", str));
      }

      self.push_pos();
      let mut strm = Stream::new(str);
      while ! self.eof() && ! strm.eof() {
         if self.next_char_or_fail() != strm.next_char_or_fail() {
            self.pop_and_reset_pos();
            return Err(format!("Couldn't skip str '{}'!", str));
         }
      }

      self.pop_pos();
      Ok(())
   }

   pub fn skip_while<F>(&mut self, test: F) where
      F: Fn(char) -> bool
   {
      while ! self.eof() {
         self.push_pos();
         if ! test(self.next_char_or_fail()) {
            self.pop_and_reset_pos();
            break;
         }

         self.pop_pos();
      }
   }

   pub fn skip_whitespace(&mut self)
   {
      self.skip_while(|c| c.is_whitespace());
   }

   pub fn take_while<F>(&mut self, test: F) -> String where
      F: Fn(char) -> bool
   {
      let mut string = String::new();
      while ! self.eof() {
         self.push_pos();
         let char = self.next_char_or_fail();
         if test(char) {
            string.push(char);
            self.pop_pos();
         }
         else {
            self.pop_and_reset_pos();
            break;
         }
      }

      string
   }

   pub fn take_uint(&mut self) -> Result<usize, ParseError>
   {
      self.push_pos();
      let mut digits = String::new();
      while ! self.eof() {
         self.push_pos();
         let char = self.next_char_or_fail();
         if char.is_digit(10) {
            digits.push(char);
            self.pop_pos();
         }
         else {
            self.pop_and_reset_pos();
            break;
         }
      }

      match digits.parse::<usize>() {
         Ok(usize) => {
            self.pop_pos();
            Ok(usize)
         }

         Err(err) => {
            self.pop_and_reset_pos();
            Err(format!("Couldn't take usize: {}", err))
         }
      }
   }

   pub fn take_char(&mut self) -> Result<char, ParseError>
   {
      if self.eof() {
         return Err("Couldn't take char!".to_string());
      }

      Ok(self.next_char_or_fail())
   }

   pub fn take_till_eof(&mut self) -> String
   {
      let mut string = String::new();
      while ! self.eof() {
         string.push(self.next_char_or_fail());
      }

      string
   }

   #[cfg(test)]
   pub fn unconsumed(&self) -> String { self.strm.unconsumed() }

   fn next_char_or_fail(&mut self) -> char { self.strm.next_char_or_fail() }

   fn push_pos(&mut self) { self.strm.push_pos(); }

   fn pop_pos(&mut self) { self.strm.pop_pos(); }

   fn pop_and_reset_pos(&mut self) { self.strm.pop_and_reset_pos(); }
}

#[test]
#[cfg(test)]
fn tests()
{
   use std::io::{self, Write};

   match parser_tests() {
      Ok(_)    => {}
      Err(err) => writeln!(&mut io::stderr(), "Parser test error: {}", err).unwrap()
   }
}

#[cfg(test)]
fn parser_tests() -> Result<(), ParseError>
{
   let mut parser = Parser::new("ssss 21 qqq aswe ");
   assert_eq!(parser.unconsumed(), "ssss 21 qqq aswe ");

   parser.skip_whitespace();
   assert_eq!(parser.unconsumed(), "ssss 21 qqq aswe ");

   try!(parser.skip("ss"));
   assert_eq!(parser.unconsumed(), "ss 21 qqq aswe ");

   try!(parser.skip("ss"));
   assert_eq!(parser.unconsumed(), " 21 qqq aswe ");

   parser.skip_whitespace();
   assert_eq!(parser.unconsumed(), "21 qqq aswe ");

   assert_eq!(try!(parser.take_uint()), 21);
   assert_eq!(parser.unconsumed(), " qqq aswe ");

   parser.skip_whitespace();
   assert_eq!(parser.take_while(|c| c == 'q'), "qqq".to_string());
   assert_eq!(parser.unconsumed(), " aswe ");

   parser.skip_while(|c| c.is_whitespace());
   assert_eq!(parser.unconsumed(), "aswe ");

   assert_eq!(try!(parser.take_char()), 'a');
   assert_eq!(parser.unconsumed(), "swe ");

   assert_eq!(parser.eof(), false);
   assert_eq!(parser.take_till_eof(), "swe ".to_string());
   assert_eq!(parser.eof(), true);

   Ok(())
}
