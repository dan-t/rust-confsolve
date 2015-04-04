use std::str::Chars;

pub struct Stream<'a>
{
   chars: Chars<'a>,
   pos_stack: Vec<Chars<'a>>,
}

impl<'a> Stream<'a>
{
   pub fn new(input: &str) -> Stream
   {
      Stream { 
         chars: input.chars(),
         pos_stack: Vec::new()
      }
   }

   pub fn eof(&self) -> bool { self.chars.size_hint().0 <= 0 }

   pub fn next_char_or_fail(&mut self) -> char
   {
      self.chars.next().unwrap()
   }

   pub fn push_pos(&mut self) 
   {
      self.pos_stack.push(self.chars.clone());
   }

   pub fn pop_pos(&mut self) { self.pos_stack.pop(); }

   pub fn pop_and_reset_pos(&mut self) 
   { 
      if let Some(chars) = self.pos_stack.pop() {
         self.chars = chars;
      }
   }

   #[cfg(test)]
   pub fn unconsumed(&self) -> String
   {
      let chars = self.chars.clone();
      chars.collect()
   }
}

#[test]
#[cfg(test)]
fn tests()
{
   let mut strm = Stream::new("qqq www ttt");
   assert_eq!(strm.unconsumed(), "qqq www ttt".to_string());

   strm.next_char_or_fail();
   assert_eq!(strm.unconsumed(), "qq www ttt".to_string());

   strm.next_char_or_fail();
   strm.next_char_or_fail();
   strm.next_char_or_fail();
   assert_eq!(strm.unconsumed(), "www ttt".to_string());

   strm.push_pos();
   strm.next_char_or_fail();
   strm.pop_pos();
   assert_eq!(strm.unconsumed(), "ww ttt".to_string());

   strm.push_pos();
   strm.next_char_or_fail();
   strm.pop_and_reset_pos();
   assert_eq!(strm.unconsumed(), "ww ttt".to_string());

   assert_eq!(strm.next_char_or_fail(), 'w');
   assert_eq!(strm.next_char_or_fail(), 'w');

   assert_eq!(strm.next_char_or_fail(), ' ');

   strm.push_pos();
   strm.next_char_or_fail();
   strm.next_char_or_fail();
   assert_eq!(strm.unconsumed(), "t".to_string());

   assert_eq!(strm.eof(), false);
   strm.next_char_or_fail();
   assert_eq!(strm.eof(), true);
   assert_eq!(strm.unconsumed(), "".to_string());

   strm.pop_and_reset_pos();
   assert_eq!(strm.eof(), false);
   assert_eq!(strm.unconsumed(), "ttt".to_string());
}
