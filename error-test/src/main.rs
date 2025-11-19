use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::error::Error;

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn main() -> Result<(), Box<dyn Error>> {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    let greeting_fileII = File::open("helloII.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("helloII.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    let greeting_fileIII = File::open("hello.txt").unwrap();

    let greeting_fileIV = File::open("hello.txt")
        .expect("hello txt should be included in this project.");

    let greeting_file = File::open("hello.txt")?;
    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_fileII() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_fileIII() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_fileIV() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}