mod prueba;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main(){
    println!("Words Counter");

    let lines = read_lines("./palabras.txt");

    match lines {
        Ok(lines) => {
            let  mut words_matches: HashMap<String, u32> = HashMap::new();
            for line in lines.flatten() {
                    let lower_case_text: String = line.to_lowercase();
                    let words = lower_case_text.split(" ");
                    for word in words {
                        //https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry
                        //https://doc.rust-lang.org/std/borrow/trait.ToOwned.html
                        let mut key_str: String = word.to_owned();
                        key_str.retain(|c| !r#"(),".;:'"#.contains(c)); // In place replace
                        words_matches.entry(key_str)
                            .and_modify(|m| { *m += 1 })
                            .or_insert(1);
                    }
            }
            let mut sorted_collection: Vec<(&String, &u32)> = words_matches.iter().collect(); //Iter returns a tuple
            sorted_collection.sort_by(|a, b| b.1.cmp(a.1));
            for (key, val) in sorted_collection {
                println!("'{}': {} times", key, val)
            }

        },
        Err(err) => println!("Error, {}", err)
    }

}
