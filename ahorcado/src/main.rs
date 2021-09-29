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

 **/
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn get_letter() -> char {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    input.chars().nth(0).unwrap()
}

fn print_word(result_word: &Vec<String>) {
    println!("{}", result_word.join(""));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn print_wrong_characters(hash_wrong_letters: &HashMap<char, ()>) -> () {
    let characters = hash_wrong_letters.keys().map(|s| s.to_string()).collect::<Vec<_>>().join(", ");
    println!("{}", characters);
}

fn do_turn(guess_word: &String) {
    let mut remaining_attempts = 5;
    let mut guessed: Vec<String> = vec![String::from("_"); guess_word.len()];
    println!("Siguiente palabra");
    let mut bad_chars = HashMap::new();
    let mut used_chars = HashMap::new();

    for _i in 0..5 {
        println!("Ingresa una letra");
        let mut input_letter = get_letter();
        let mut found: bool = false;

        while used_chars.contains_key(&input_letter) {
            println!("La letra ingreada ya fue utilizada.");
            println!("Ingresa una letra");
            input_letter = get_letter();
        }

        for (i, c) in (guess_word).chars().enumerate() {
            if c == input_letter {
                found = true;
                guessed[i] = c.to_string();
            }
        }

        used_chars.insert(input_letter, ());

        if found {
            println!("Adivinaste las siguientes letras: {}", input_letter);
        } else {
            bad_chars.insert(input_letter, ());
        }

        if guessed.join("") == *guess_word {
            println!("Usted adivino correctamente toda la palabra: ");
            print_word(&guessed);
            break;
        } else if remaining_attempts == 1 {
            println!("Perdio. La palabra era {}", guess_word);
            println!("Letras mal utilizadas");
            print_wrong_characters(&bad_chars);
        } else {
            println!("La palabra hasta el momento es: ");
            print_word(&guessed);
        }

        remaining_attempts = remaining_attempts - 1;
    }
}

fn main(){
    println!("Bienvenido al ahorcado de FIUBA!");
    let lines = read_lines("./palabras.txt");

    match lines {
        Ok(lines) => {
            for line in lines {
                if let Ok(guess_word) = line {
                    do_turn(&guess_word);
                }
            }
        },
        Err(err) => println!("Error, {}", err)
    }

}
