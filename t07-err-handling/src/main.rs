// use std::net::IpAddr;

fn main() {
    // let home: IpAddr = "127.0.0.1".parse().expect("Hardcoded IP Address should be valid");
}

// use std::error::Error;
// use std::fs::File;

// fn main() -> Result<(), Box<dyn Error>> {
//     let greeting_file = File::open("hello.txt")?;
//     Ok(())
// }

// use std::fs;
// use std::io;
// use std::io::ErrorKind;
// use std::io::{self, Read};

// fn main() {
//     // panic!("destroy");

//     // let v = vec![1, 2, 3];
//     // v[99];

//     // let greeting_file_result = File::open("hello.txt");
//     // let greeting_file = match greeting_file_result {
//     //     Ok(file) => file,
//     //     Err(e) => match e.kind() {
//     //         ErrorKind::NotFound => match File::create("hello.txt") {
//     //             Ok(fc) => fc,
//     //             Err(_e) => panic!("Problem creating the file : {_e:?}")
//     //         }, 
//     //         _ => panic!("Problem opening the file: {e:?}")
//     //     }
//     // };

//     // let greeting_file = File::open("hello.txt").unwrap_or_else(|e| {
//     //     if e.kind() == ErrorKind::NotFound {
//     //         File::create("hello.txt").unwrap_or_else(|_e| panic!("Problem creating the file: {_e:?}"))
//     //     } else {
//     //         panic!("Problem opening the file: {e:?}");
//     //     }
//     // });

//     // let greeting_file = File::open("hello.txt")
//     //     .expect("hello.txt should be included in the project.");

//     // fn read_username_from_file() -> Result<String, io::Error> {
//     //     // let username_file_result = File::open("hello.txt");
//     //     // let mut username_file = match username_file_result {
//     //     //     Ok(file) => file,
//     //     //     Err(e) => return Err(e)
//     //     // };
//     //     // let mut username = String::new();
//     //     // match username_file.read_to_string(&mut username) {
//     //     //     Ok(_) => Ok(username),
//     //     //     Err(e) => Err(e)
//     //     // }

//     //     // let mut username_file = File::open("hello.txt")?;
//     //     // let mut username = String::new();
//     //     // username_file.read_to_string(&mut username)?;
//     //     // Ok(username)

//     //     // let mut username = String::new();
//     //     // File::open("hello.txt")?.read_to_string(&mut username)?;
//     //     // Ok(username) 

//     //     fs::read_to_string("hello.txt")
//     // }
// }
