pub struct Stream<'a>
{
   pos      :  uint,
   input    :  &'a str,
   pos_stack:  Vec<uint>
}

impl<'a> Stream<'a>
{
   pub fn new(input: &str) -> Stream
   {
      Stream {pos: 0u, input: input, pos_stack: Vec::new()}
   }

   pub fn eof(&self) -> bool { self.pos >= self.input.len() }

   pub fn take_char_or_fail(&mut self) -> char 
   {
      let range = self.input.char_range_at(self.pos);
      self.pos = range.next;
      return range.ch;
   }

   pub fn next_char_or_fail(&self) -> char { self.input.char_at(self.pos) }

   pub fn push_pos(&mut self) { self.pos_stack.push(self.pos); }

   pub fn pop_pos(&mut self) { self.pos_stack.pop(); }

   pub fn pop_and_reset_pos(&mut self) { self.pos_stack.pop().map(|p| self.pos = p); }

   #[cfg(test)]
   pub fn consumed(&self) -> &str
   {
      if self.pos >= self.input.len() {
         self.input
      }
      else {
         self.input.slice_to_or_fail(&self.pos)
      }
   }

   #[cfg(test)]
   pub fn unconsumed(&self) -> &str 
   {
      if self.pos >= self.input.len() {
         ""
      }
      else {
         self.input.slice_from_or_fail(&self.pos)
      }
   }
}

#[test]
#[cfg(test)]
fn tests()
{
   let mut strm = Stream::new("qqq www ttt");
   assert_eq!(strm.consumed(), "");
   assert_eq!(strm.unconsumed(), "qqq www ttt");

   strm.take_char_or_fail();
   assert_eq!(strm.consumed(), "q");
   assert_eq!(strm.unconsumed(), "qq www ttt");

   strm.take_char_or_fail();
   strm.take_char_or_fail();
   strm.take_char_or_fail();
   assert_eq!(strm.consumed(), "qqq ");
   assert_eq!(strm.unconsumed(), "www ttt");

   strm.push_pos();
   strm.take_char_or_fail();
   strm.pop_pos();
   assert_eq!(strm.consumed(), "qqq w");
   assert_eq!(strm.unconsumed(), "ww ttt");

   strm.push_pos();
   strm.take_char_or_fail();
   strm.pop_and_reset_pos();
   assert_eq!(strm.consumed(), "qqq w");
   assert_eq!(strm.unconsumed(), "ww ttt");

   assert_eq!(strm.next_char_or_fail(), 'w');
   assert_eq!(strm.next_char_or_fail(), 'w');

   strm.take_char_or_fail();
   strm.take_char_or_fail();
   assert_eq!(strm.next_char_or_fail(), ' ');

   strm.push_pos();
   strm.take_char_or_fail();
   strm.take_char_or_fail();
   strm.take_char_or_fail();
   assert_eq!(strm.consumed(), "qqq www tt");
   assert_eq!(strm.unconsumed(), "t");

   assert_eq!(strm.eof(), false);
   strm.take_char_or_fail();
   assert_eq!(strm.eof(), true);
   assert_eq!(strm.consumed(), "qqq www ttt");
   assert_eq!(strm.unconsumed(), "");

   strm.pop_and_reset_pos();
   assert_eq!(strm.eof(), false);
   assert_eq!(strm.consumed(), "qqq www");
   assert_eq!(strm.unconsumed(), " ttt");
}
