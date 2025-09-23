use std::fmt::format;

pub fn learn_strings () {
    // initialize a string
    let s = String::from("Hello, world");
    println!("{s}");

    // convert from string slice to owned string
    let str_slice = "Hello from Chef Rustoshi";
    let owned_string = str_slice.to_owned();
    let owned_string2 = str_slice.to_string();

    println!("owned string 1 {}", owned_string);
    println!("owned string 1 {}", owned_string2);

    // updating a string
    let mut full_name = String::from("Rustoshi");
    full_name.push_str(" Dev");

    println!("My name is {}", full_name);

    // push method takes only one char
    full_name.push('😀');

    println!("My name with emoji {}", full_name);

    // concatenation with + and format! macro
    // + adds one owned string to one or more &str slices
    let name = "Bob".to_string();
    let role = "Devrel Solana Foundation".to_owned();

    let name_with_role = name.to_owned() + " I'm a " + &role;
    println!("{name_with_role}");
    // adding with format! macro
    let name_with_role = format!("{name} i'm a {role}");
    println!("{name_with_role}");

    // indexing into strings
    let sentence = "if i paste am, credit no delay".to_owned();
    // let sentence_index = sentence[1];
    // the above line with work because strings in rust
    // are a wrapper over Vec<u8> as series of bytes, sometimes
    // a character can take up more than one byte, eg Cyrrilic chars
    // so indexing a particular char can may cause undwanted results or erros
    // and the String type does not implement the SliceIndex trait

    // correct ways to index Strings
    // 1. as bytes
    for c in sentence.bytes() {
        println!("{c}");
    }

    // 2. as chars
    for c in sentence.chars() {
        println!("{c}");
    }


}