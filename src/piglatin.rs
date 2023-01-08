pub fn pig_latin(s: &str) -> String {
  let mut answer = s.to_string();

  let _ = match answer.pop() {
      None => panic!("Empty string!"),
      Some(c) => c
  };

  let len = answer.len();
  let first_letter = first_char(&s);
  let char_consonant = is_char_consonant(first_letter);

  if char_consonant {
      answer = answer[1..len].to_string();
      answer.push(first_letter);
      answer.push_str("-ay");
  } else {
      answer.push_str("-hay");
  }
  return format!("{}", answer);
}

fn first_char(s: &str) -> char {
  let mut chars = s.chars();
  
  match chars.next() {
      None => panic!("Empty string!"),
      Some(c) => c
  }
}

fn is_char_consonant(c: char) -> bool {
  for vowel in ['a', 'e', 'i', 'o', 'u'].iter() {
    if c == *vowel {
      return false;
    }
  }
  return true;
}