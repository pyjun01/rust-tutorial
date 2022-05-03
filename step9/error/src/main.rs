use std::io;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let f = File::open("hello.txt");
    let f = File::open("hello.txt").unwrap(); // Ok라면 값을 return하고, Err라면 panic!을 호출함
    let f = File::open("hello.txt").expect("Failed to open hello.txt"); // expect는 unwrap과 동일하지만 에러 메세지를 직접 선택할 수 있음

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create file but there was a problem: {:?}", e);
                },
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        }
    };
}
