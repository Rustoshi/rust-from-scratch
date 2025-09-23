use std::{collections::HashMap, fmt::format};

pub fn learn_hashmaps() {
    let mut scores: HashMap<String, u32>  = HashMap::new();

    const NIGERIA: &str = "NIGERIA";
    const GHANA: &str = "GHANA";

    scores.insert(NIGERIA.to_owned(), 5);
    scores.insert(GHANA.to_string(), 0);

    let match_results = format!("Nigeria {} - Ghana {}", scores.get(NIGERIA).unwrap(), scores.get(GHANA).unwrap());
    println!("{match_results}");

    let nigeria_score = scores.get(NIGERIA).copied().unwrap();
    let ghana_score = scores.get(GHANA).unwrap();

    println!("Nigeria scored {}", nigeria_score);
    println!("Ghana scored {}", ghana_score);

    // iterating over hashmaps
    println!("iterating through scores...");
    for (key, value) in &scores {
        println!("{key} : {value}");
    }

}