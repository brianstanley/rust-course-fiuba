pub mod game_mod {
    use std::collections::HashMap;
    use crate::error::GameError;


    pub struct Ahorcado {
        pub word:String,
        pub remaining_attempts: i32,
        pub used_chars: HashMap<char, ()>,
        pub wrong_chars: HashMap<char, ()>,
        pub stats: Stats
    }

    pub enum GameStatus {
        Success,
        Pending,
        CharGuessed,
        GameOver
    }

    pub struct Stats{
        guessed:Vec<String>,
        pub(crate) guessed_char: char,
        pub status: GameStatus,
    }

    impl Stats {
        pub fn get_guessed_word(&self) -> String{
            self.guessed.join("")
        }
    }

    impl Ahorcado {
        pub fn new(word: String) -> Ahorcado {
            let wrong_chars = HashMap::new();
            let used_chars = HashMap::new();
            let stats: Stats = Stats{
                guessed: vec![String::from("_"); word.len()],
                status: GameStatus::Pending,
                guessed_char: '\0',
            };
            Ahorcado {
                word,
                remaining_attempts: 5,
                used_chars,
                wrong_chars,
                stats
            }
        }

        pub(crate) fn use_char(&mut self, char: char) {
            self.used_chars.insert(char, ());
        }
        pub(crate) fn add_wrong_char(&mut self, char: char) {
            self.wrong_chars.insert(char, ());
        }
        pub(crate) fn substract_remaining_attempts(&mut self) {
            self.remaining_attempts -= 1;
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
            &self.word
        }
        pub(crate) fn get_wrong_chars(&self) -> &HashMap<char, ()> {
            &self.wrong_chars
        }
        pub fn play(&mut self, char: char) -> Result<&Stats, GameError> {
            self.stats.status = GameStatus::Pending;
            let input_letter = char;
            let mut found: bool = false;


            if !self.can_play() {
                return Err(GameError::NoChancesAvailable);
            }

            if self.is_char_already_used(&input_letter) {
                return Err(GameError::CharacterIsAlreadyUsed);
            }

            for (i, c) in (self.word).chars().enumerate() {
                if c == input_letter {
                    found = true;
                    self.stats.guessed[i] = c.to_string();
                }
            }

            self.use_char(input_letter);

            if found {
                self.stats.guessed_char = input_letter;
                self.stats.status = GameStatus::CharGuessed
            } else {
                self.add_wrong_char(input_letter);
            }

            if self.stats.guessed.join("") == *self.get_word_to_guess() {
                self.stats.status = GameStatus::Success;
            }

            self.substract_remaining_attempts();

            Ok(&self.stats)
        }
    }

}