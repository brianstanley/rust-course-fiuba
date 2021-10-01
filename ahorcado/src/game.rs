pub mod game {
    use std::collections::HashMap;

    pub struct Ahorcado<'a> {
        pub word: &'a String,
        pub remaining_attempts: i32,
        pub used_chars: &'a mut HashMap<char, ()>,
        pub wrong_chars:&'a mut  HashMap<char, ()>
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

        pub(crate) fn use_char(&mut self, char: char) {
            self.used_chars.insert(char, ());
        }
        pub(crate) fn add_wrong_char(&mut self, char: char) {
            self.wrong_chars.insert(char, ());
        }
        pub(crate) fn substract_remaining_attempts(&mut self) {
            self.remaining_attempts = self.remaining_attempts - 1;
        }
        pub(crate) fn is_char_already_used(&self, char: &char) -> bool {
            self.used_chars.contains_key(char)
        }
        pub(crate) fn can_play(&self) -> bool {
            self.remaining_attempts > 1
        }
        pub(crate) fn get_remaining_attempts(&self) -> i32 {
            self.remaining_attempts
        }
        pub(crate) fn get_word_to_guess(&self) -> &String {
            self.word
        }
        pub(crate) fn get_wrong_chars(&self) -> &HashMap<char, ()> {
            self.wrong_chars
        }
    }
    
}