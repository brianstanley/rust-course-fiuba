use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, Lines};
use std::iter::Flatten;
use std::path::Path;

fn main() {
    let lines_as_vec: Result<Vec<String>, Error> = read_file_lines("test.txt".to_string());
    match lines_as_vec {
        Ok(result) => {
            println!("{:?}", result)
        }
        Err(_) => {}
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn read_file_lines(path: String) -> Result<Vec<String>, Error> {
    let reader = read_lines(path);
    return match reader {
        Ok(lines) => {
            Ok(lines.collect::<Result<_, _>>().unwrap())
        }
        Err(e) => {
            return Err(e);
        }
    }

}
