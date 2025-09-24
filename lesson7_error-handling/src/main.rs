use std::{fs::File, io::{ErrorKind, Read}};

// There are two types of erros
// 1. Recoverable errors and
// 2. Unrecoverable errors
// Recoverable errors can be managed manually because they return the Result<T, E> enum, and we can 
// decide what our program does with it, these are the kinds of errors that are not critical like accessing a file
// that does not exist
// Unrecoverable errors on the other hand are errors are errors that can cause are program to break, they are not manageable
// nor return a value, errors like indexing an array out of bounds.

fn main() {
    // Unrecoverable errors

    // panic!("Crash on purpose!"); // will panic and break our program

    // let v = vec![1,2,3,4,5];
    // let sixth_index = v[5];
    // println!("{}", sixth_index); // will panic and break our program

    // Recoverable Errors
    let document_file_result = File::open("hello.txt");

    let mut document = match document_file_result {
        Err(e) => match e.kind() {
            ErrorKind::NotFound => panic!("File not found"),
            _ => panic!("Error opening document: {:#?}", e)
        },
        Ok(d) => d
    };

    let mut contents = String::new();

    document.read_to_string(&mut contents).expect("Error reading contents");

    println!("Contents of the documents below \n {}", contents);

    use std::net::IpAddr;

}
