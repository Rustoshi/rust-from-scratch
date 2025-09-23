pub fn learn_vectors() {
        // WORKING WITH VECTORS
        let mut v: Vec<i32> = Vec::new();

        v.push(23);
        v.push(12392);
        v.push(382);
    
    
        let first = v.get(0).expect("Index out of bounds");
        let second = v[1];
    
        println!("First item on the vector is {}", first);
        println!("Secod item on the vector is {}", second);
    
        let mut second_again = v[1];
    
        println!("Second again is {}", second_again);
    
        second_again = 10;
    
        let second_from_vec = v[1];
    
        println!("Second again is {}", second_again);
        println!("Second from vec is {}", second_from_vec);
    
        let third = &v[2];
    
        println!("Third item is {}", third);
    
        // iteration over vectors
        for i in &v {
            println!("item - {}", i);
        }
    
        println!("Before mutable iteration - {:#?}", &v);
    
        // iterating over mutable vectors
        for j in &mut v{
            *j *= 2;
        }
    
        println!("After mutable iteration - {:#?}", &v);
    
        // combining enum and vectors to store multiple types in a vector
        #[derive(Debug)]
        enum Password {
            Int(i32),
            String(String)
        }
    
        let mut passwords: Vec<Password> = Vec::new();
    
        passwords.push(Password::Int(32939329));
        passwords.push(Password::String(String::from("password123")));
    
        // get individual items
        if let Password::Int(v) = passwords[0] {
            println!("Number password is {}", v);
        }
    
        // use for loop and match to get all values
        println!("Looping through items in multi-typed vector");
        for item in passwords {
            match item {
                Password::Int(value) => println!("{value}"),
                Password::String(value) => println!("{value}")
            }
        }
    
        // initializing a vector with capacity
        let mut vec_with_capacity = Vec::with_capacity(10);
        
        for i in 1..20 {
            vec_with_capacity.push(i);
        }
    
        println!("Vector with capacity {:#?}", vec_with_capacity);
   
}