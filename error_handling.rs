use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

fn main() {}

fn readfile() {
    let f: Result<String, io::Error> = File::open("hello.txt");

    let f: File = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => println!("There was a problem creating file {}", e),
            },
            other_err => {
                panic!("problem openining the file ")
            }
        },
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s: String = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
