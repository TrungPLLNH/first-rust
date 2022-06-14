use std::io;

fn main() {
  let my_string: &str = "This is a Nregular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.";
  
  let mut entered_text: String = String::new();
  io::stdin().read_line(&mut entered_text).unwrap();
  entered_text.pop();

  println!("{}", count_sub_string(my_string, &entered_text[..]));
}

fn count_sub_string(string: &str, sub_string: &str) -> u32 {
  let string_lowercase = string.to_lowercase();
  return *&string_lowercase[..].matches(&sub_string.to_lowercase()).count() as u32;
}