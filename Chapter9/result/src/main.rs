use std::fs::File;
use std::io::Read;

fn test_result_branch() {
    // handling error branch with match
    // let greeting_file = match File::open("hello.txt") {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         std::io::ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(file) => file,
    //             Err(other_error) => panic!("Problem creating the file: {:?}", other_error),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         },
    //     },
    // };

    // handle error branch with unwrap_or_else
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error)
            })
        }
        else {
            panic!("Problem opening the file: {:?}", error)
        }
    });
    println!("greeting file: {:?}", greeting_file);

    // handle error branch with expect
    // let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");
}

fn error_propagate() -> Result<String, std::io::Error> {
    // using match to return err or deal ok
    let mut s = String::new();
    match File::open("hello2.txt") {
        Ok(mut file) => match file.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(error) => Err(error),
        }
        Err(error) => Err(error),
    }
}

fn error_propagate2() -> Result<String, std::io::Error> {
    // using ? to return err or deal ok
    let mut s = String::new();
    let mut file = File::open("hello3.txt")?;
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn test_error_propagate() -> Result<String, std::io::Error> {
    let s = error_propagate();
    println!("s1 {:?}", s);
    let s = error_propagate2()?;
    println!("s2 {:?}", s);
    Ok(String::from("test_error_propagate"))
}

fn main() -> Result<(), std::io::Error> { // this return type implements the std::process::Termination trait
    test_result_branch();
    test_error_propagate()?;
    Ok(())
}
