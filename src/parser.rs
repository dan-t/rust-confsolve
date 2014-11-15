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

   pub fn skip(&mut self, str: &str) -> Result<(), ParseError>
   {
      if str.is_empty() {
         return Err("Couldn't skip empty str!".to_string());
      }

      self.push_pos();
      let mut strm = Stream::new(str);
      while ! self.eof() && ! strm.eof() {
         if self.next_char() != strm.next_char() {
            self.pop_and_reset_pos();
            return Err(format!("Couldn't skip str '{}'!", str));
         }

         self.take_char_();
         strm.take_char();
      }

      self.pop_pos();
      Ok(())
   }

   pub fn skip_while(&mut self, test: |char| -> bool)
   {
      while ! self.eof() && test(self.next_char()) {
         self.take_char_();
      }
   }

   pub fn skip_whitespace(&mut self)
   {
      self.skip_while(|c| c.is_whitespace());
   }

   pub fn take_while(&mut self, test: |char| -> bool) -> Result<String, ParseError>
   {
      let mut string = String::new();
      while ! self.eof() {
         if ! test(self.next_char()) {
            return Ok(string);
         }

         string.push(self.take_char_());
      }

      Ok(string)
   }

   pub fn take_uint(&mut self) -> Result<uint, ParseError>
   {
      self.push_pos();
      let mut digits = String::new();
      while ! self.eof() && self.next_char().is_digit() {
         digits.push(self.take_char_());
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

      Ok(self.take_char_())
   }

   pub fn take_till_eof(&mut self) -> Result<String, ParseError>
   {
      let mut string = String::new();
      while ! self.eof() {
         string.push(self.take_char_());
      }

      Ok(string)
   }

   pub fn eof(&self) -> bool { self.strm.eof() }

   fn take_char_(&mut self) -> char { self.strm.take_char() }

   fn next_char(&self) -> char { self.strm.next_char() }

   fn push_pos(&mut self) { self.strm.push_pos(); }

   fn pop_pos(&mut self) { self.strm.pop_pos(); }

   fn pop_and_reset_pos(&mut self) { self.strm.pop_and_reset_pos(); }
}

struct Stream<'a>
{
   pos      :  uint,
   input    :  &'a str,
   pos_stack:  Vec<uint>
}

impl<'a> Stream<'a>
{
   fn new(input: &str) -> Stream
   {
      Stream {pos: 0u, input: input, pos_stack: Vec::new()}
   }

   fn eof(&self) -> bool { self.pos >= self.input.len() }

   fn take_char(&mut self) -> char 
   {
      let range = self.input.char_range_at(self.pos);
      self.pos = range.next;
      return range.ch;
   }

   fn next_char(&self) -> char { self.input.char_at(self.pos) }

   fn push_pos(&mut self) { self.pos_stack.push(self.pos); }

   fn pop_pos(&mut self) { self.pos_stack.pop(); }

   fn pop_and_reset_pos(&mut self) { self.pos_stack.pop().map(|p| self.pos = p); }
}
