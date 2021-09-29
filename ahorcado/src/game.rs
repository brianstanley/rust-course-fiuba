pub mod game {
    use std::collections::HashMap;

    pub struct Ahorcado<'a> {
        pub word: &'a String,
        pub remaining_attempts: i32,
        pub used_chars: &'a mut HashMap<char, ()>,
        pub wrong_chars:&'a mut  HashMap<char, ()>
    }

    impl<'a> Ahorcado<'a> {
        pub(crate) fn use_char(&mut self, char: char) {
            self.used_chars.insert(char, ());
        }
    }

    impl<'a> Ahorcado<'a> {
        pub(crate) fn is_char_already_used(&self, char: &char) -> bool {
            self.used_chars.contains_key(char)
        }
    }

    impl Ahorcado <'_> {
        pub fn new<'a>(
            word: &'a String,
            remaining_attempts: i32,
            used_chars: &'a mut HashMap<char, ()>,
            wrong_chars: &'a mut HashMap<char, ()>) -> Ahorcado<'a> {
            Ahorcado {
                word,
                remaining_attempts,
                used_chars,
                wrong_chars
            }
        }
    }
    
}