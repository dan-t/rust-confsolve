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

         self.take_char_or_fail();
         strm.take_char_or_fail();
      }

      self.pop_pos();
      Ok(())
   }

   pub fn skip_while(&mut self, test: |char| -> bool)
   {
      while ! self.eof() && test(self.next_char_or_fail()) {
         self.take_char_or_fail();
      }
   }

   pub fn skip_whitespace(&mut self)
   {
      self.skip_while(|c| c.is_whitespace());
   }

   pub fn take_while(&mut self, test: |char| -> bool) -> String
   {
      let mut string = String::new();
      while ! self.eof() {
         if ! test(self.next_char_or_fail()) {
            return string;
         }

         string.push(self.take_char_or_fail());
      }

      string
   }

   pub fn take_uint(&mut self) -> Result<uint, ParseError>
   {
      self.push_pos();
      let mut digits = String::new();
      while ! self.eof() && self.next_char_or_fail().is_digit(10) {
         digits.push(self.take_char_or_fail());
      }

      match from_str::<uint>(digits.as_slice()) {
         Some(uint) => {
            self.pop_pos();
            Ok(uint)
         }

         None => {
            self.pop_and_reset_pos();
            Err("Couldn't take uint!".to_string())
         }
      }
   }

   pub fn take_char(&mut self) -> Result<char, ParseError>
   {
      if self.eof() {
         return Err("Couldn't take char!".to_string());
      }

      Ok(self.take_char_or_fail())
   }

   pub fn take_till_eof(&mut self) -> String
   {
      let mut string = String::new();
      while ! self.eof() {
         string.push(self.take_char_or_fail());
      }

      string
   }

   #[cfg(test)]
   pub fn consumed(&self) -> &str { self.strm.consumed() }

   #[cfg(test)]
   pub fn unconsumed(&self) -> &str { self.strm.unconsumed() }

   fn take_char_or_fail(&mut self) -> char { self.strm.take_char_or_fail() }

   fn next_char_or_fail(&self) -> char { self.strm.next_char_or_fail() }

   fn push_pos(&mut self) { self.strm.push_pos(); }

   fn pop_pos(&mut self) { self.strm.pop_pos(); }

   fn pop_and_reset_pos(&mut self) { self.strm.pop_and_reset_pos(); }
}

#[test]
#[cfg(test)]
fn tests()
{
   use std::io;

   match parser_tests() {
      Ok(_)    => {}
      Err(err) => {
         let stderr = &mut io::stderr();
         let _ = writeln!(stderr, "Parser test error: {}", err);
      }
   }
}

#[cfg(test)]
fn parser_tests() -> Result<(), ParseError>
{
   let mut parser = Parser::new("ssss 21 qqq aswe ");
   assert_eq!(parser.consumed(), "");
   assert_eq!(parser.unconsumed(), "ssss 21 qqq aswe ");

   parser.skip_whitespace();
   assert_eq!(parser.consumed(), "");
   assert_eq!(parser.unconsumed(), "ssss 21 qqq aswe ");

   try!(parser.skip("ss"));
   assert_eq!(parser.consumed(), "ss");
   assert_eq!(parser.unconsumed(), "ss 21 qqq aswe ");

   try!(parser.skip("ss"));
   assert_eq!(parser.consumed(), "ssss");
   assert_eq!(parser.unconsumed(), " 21 qqq aswe ");

   parser.skip_whitespace();
   assert_eq!(parser.consumed(), "ssss ");
   assert_eq!(parser.unconsumed(), "21 qqq aswe ");

   assert_eq!(try!(parser.take_uint()), 21u);
   assert_eq!(parser.consumed(), "ssss 21");
   assert_eq!(parser.unconsumed(), " qqq aswe ");

   parser.skip_whitespace();
   assert_eq!(parser.take_while(|c| c == 'q'), "qqq".to_string());
   assert_eq!(parser.consumed(), "ssss 21 qqq");
   assert_eq!(parser.unconsumed(), " aswe ");

   parser.skip_while(|c| c.is_whitespace());
   assert_eq!(parser.consumed(), "ssss 21 qqq ");
   assert_eq!(parser.unconsumed(), "aswe ");

   assert_eq!(try!(parser.take_char()), 'a');
   assert_eq!(parser.consumed(), "ssss 21 qqq a");
   assert_eq!(parser.unconsumed(), "swe ");

   assert_eq!(parser.eof(), false);
   assert_eq!(parser.take_till_eof(), "swe ".to_string());
   assert_eq!(parser.eof(), true);

   Ok(())
}
