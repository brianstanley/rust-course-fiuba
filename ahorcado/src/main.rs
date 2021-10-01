/**
El objetivo del ejercicio es implementar un programa de consola para jugar al ahorcado.


Bienvenido al ahorcado de FIUBA!

La palabra hasta el momento es: _ _ _ _ _ _
Adivinaste las siguientes letras:
Te quedan 5 intentos.
Ingresa una letra: r

La palabra hasta el momento es: _ _ _ _ _ r
Adivinaste las siguientes letras: r
Te quedan 5 intentos.
Ingresa una letra: c
Si se ingresa una letra que no forma parte de la palabra, se pierde un intento.

La lista de palabras se debe leer de un archivo de texto, donde cada línea del archivo contendrá una palabra. De esa lista, se deberá elegir una palabra (puede ser una selección secuencial de palabras).

El programa termina cuando se adivina correctamente la palabra pensada, o cuando se acabaron los intentos.

Mostrar las letras que se ingresaron y que no forman parte de la palabra (las que hacen que se pierda un intento).

Verificar si se ingresó nuevamente una letra que ya estaba.
 **/
mod game;
mod error;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::error::GameError;
use crate::game::game_mod::{Ahorcado, GameStatus};

fn get_char() -> char {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.chars().next().unwrap()
}

fn print_word(result_word: &[String]) {
    println!("{}", result_word.join(""));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn print_wrong_characters(hash_wrong_chars: &HashMap<char, ()>) {
    let characters = hash_wrong_chars.keys().map(|s| s.to_string()).collect::<Vec<_>>().join(", ");
    println!("{}", characters);
}


fn play(ahorcado: &mut Ahorcado) {

    loop {
        println!("turnos restantes: {}", ahorcado.get_remaining_attempts());
        println!("Ingresa una letra");
        let input_char = get_char();
        let game = ahorcado.play(input_char);
        match &game  {
                Ok(stats) => {
                    match stats.status {
                        GameStatus::Success => {
                            println!("Felicitaciones adivino la palabra");
                            println!("La palabra era: {}", ahorcado.word);
                            break;
                        }
                        GameStatus::Pending => {
                            println!("Mal!!.");
                        }
                        GameStatus::GameOver => {
                            println!("Game over");
                            break;
                        }
                        GameStatus::CharGuessed => {
                            println!("Adivino la siguiente letra: {}", stats.guessed_char);
                            println!("La palabra hasta ahora es: {}", stats.get_guessed_word());
                        }
                    }
                },
                Err(e) => {
                    match e {
                        GameError::NoChancesAvailable => {
                            eprintln!("Error: {}", e);
                            break;
                        },
                        GameError::CharacterIsAlreadyUsed => eprintln!("Error: {}", e),
                    }
                }
            }
    }

}

fn main(){
    println!("Bienvenido al ahorcado de FIUBA!");

    let lines = read_lines("./palabras.txt");

    match lines {
        Ok(lines) => {
            for line in lines {
                if let Ok(guess_word) = line {
                    let mut ahorcado = game::game_mod::Ahorcado::new(guess_word);
                    play(&mut ahorcado);
                }
            }
        },
        Err(err) => println!("Error, {}", err)
    }

}
