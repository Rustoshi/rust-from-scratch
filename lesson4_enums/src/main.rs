fn main() {
    enum IpAddrrKind {
        v4,
        v6
    }

    let four = IpAddrrKind::v4;
    let six = IpAddrrKind::v6;

    // we can put data directly into an enum variant
    #[derive(Debug)]
    enum IpAddrKind2 {
        v4(String),
        v6(String)
    }

    let four2 = IpAddrKind2::v4(String::from("127.0.0.1"));
    let six2 = IpAddrKind2::v6(String::from("::1"));

    println!("{:#?}", four2);

    // Pattern Matching
    enum Languages {
        Javascript,
        Python,
        Rust,
        Java,
        Swift
    }

    fn useCase(lan: Languages) -> String {
        match lan {
            Languages::Javascript => "For web developent".to_string(),
            Languages::Python => "For AI/ML".to_string(),
            Languages::Rust => "For Solana development".to_string(),
            Languages::Java => "For Android development".to_string(),
            Languages::Swift => "For IOS development".to_string()
        }
    }

    let my_usecase = useCase(Languages::Rust);
    println!("Rust is used for {}", my_usecase);
}
