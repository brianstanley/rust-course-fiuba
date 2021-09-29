pub mod game {
    use std::collections::HashMap;

    pub struct Ahorcado<'a> {
        pub word: &'a String,
        pub remaining_attempts: i32,
        pub used_chars: &'a mut HashMap<char, ()>,
        pub wrong_chars:&'a mut  HashMap<char, ()>
    }

    impl Ahorcado <'_> {
        pub fn new<'a>(word: &'a String, remaining_attempts: i32, used_chars: &'a mut HashMap<char, ()>, wrong_chars: &'a mut HashMap<char, ()>) -> Ahorcado<'a> {
            Ahorcado {
                word,
                remaining_attempts,
                used_chars,
                wrong_chars
            }
        }
    }
    
}