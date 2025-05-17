use std::fs::File;
use std::io::ErrorKind;

fn main() {
    read_or_create_greeting_file();
    // try_read_greeting_file();
    // try_read_greeting_file_if_let();
}

fn try_read_greeting_file() -> File {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
    return greeting_file;
}

fn read_or_create_greeting_file() -> File {
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
    return greeting_file;
}
// fn try_read_greeting_file_if_let() -> File {
//     if let Ok(file) = File::open("hello.txt") {
//         return file;
//     } else {
//         panic!("Problem opening the file.")
//     }
// }
