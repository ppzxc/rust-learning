use std::fs::File;
use std::io;
use std::io::ErrorKind;

pub fn panic() {
    // panic!("Crash And Burn")
    let file = File::open("data");
    // enum type panic control
    let _file = match file {
        Ok(f) => f,
        Err(error) => panic!("Failed to open the file: {:?}", error),
    };

    // ------------------------------------------------

    let file2 = File::open("data");
    // if
    let _file = match file2 {
        Ok(ff) => ff,
        Err(error) => match error.kind() {
            // NotFound 일때, data Create
            ErrorKind::NotFound => match File::create("data") {
                Ok(fc) => fc,
                Err(e) => panic!("Failed to create file: {:?}", e),
            },
            other_error => panic!("Failed to open file: {:?}", other_error),
        },
    };

    // ------------------------------------------------

    // unwrap, Ok to 'Ok' or Err to Panic
    let _file = File::open("data2").unwrap();

    // similar unwrap
    let _file = File::open("data3").expect("Failed to open the data file");
}

fn _open_file() -> Result<File, io::Error> {
    let file = File::open("data")?; // ? is throw err to call method
                                    // do stuff with `file`
    Ok(file)
}
