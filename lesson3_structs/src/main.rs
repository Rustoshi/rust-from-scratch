fn main() {
    // regular struct
    struct User {
        name: String,
        email: String,
        balance: u64
    }

    let user1 = User {
        name: "Alice".to_string(),
        email: "alice@gmail.com".to_string(),
        balance: 25_000_000
    };

    let mut user2 = User {
        name: "Bob".to_string(),
        email: "bob@gmail.com".to_string(),
        balance: 1_000
    };

    user2.balance = 500_000;

    println!("Alice's balance is {}, Bob's balance is {}", user1.balance, user2.balance);


    // Tuple Structs
    struct UserS(String, String, u64);

    let user3 = UserS(String::from("Jane"), String::from("jane@gmail.com"), 340_000);

    println!("Jane's balance is {}", user3.2);

    // Unit struct - a type of struct that does not contain any field but can also have traits impl.
    struct EmptyStruct;

    // Method Syntax
    #[derive(Debug)]
    struct Programmer {
        name: String,
        language: String
    }

    impl Programmer {
        fn new(&self, name: String, language: String) -> Self {
            return Self {
                name,
                language
            }
        }
    }

    let p1 = Programmer::new(String::from("Rustoshidev"), String::from("Rust"));

    println!("{:#?}", p1);
    
}
