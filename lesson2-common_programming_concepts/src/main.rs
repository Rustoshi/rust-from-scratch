use std::io;

fn learn_nunmber() {

    // floating point division gives precise results
    let a = 5.0;
    let b = 20.5;
    let resultab = b / a;
    // println!("{}", resultab);

    // integers gives truncated results
    let c = 10;
    let d = 4;
    let resultcd = c / d;
    // println!("{}", resultcd);

    // modulos returns remainder from division
    let total = 2000;
    let divider = 9;
    let remainder = total % divider;
    // println!("{}", remainder);
}

fn learn_arrays () {
    // items in an array must have the same type and a fixed length
    let my_arr1 = [1,2,3,4,5];
    // you can annoted an array declaration with the type and lenth in a square bracket
    let my_arr2: [u16; 3] = [23, 12, 29];
    // we can initialize an array with a fixed length to contain the same value initially
    let my_arr3 = [3; 5];
    // elements of an array can be accessed using indexes
    let first_item = my_arr3[0];
    // println!("First item of array 3 is {}", first_item);

    // using std io to get item from array based on index
    let my_arr4 = [12,44,38,65,43,23,2,34,5,9,30];

    println!("Enter index: ");
    let mut index = "".to_string();

    io::stdin()
    .read_line(&mut index)
    .expect("Please provide an input");

    let index: u16 = index.trim().parse().expect("Please enter a valide number");

    if index > my_arr4.len() as u16 {
        println!("Index should be between 0 - {}", my_arr4.len());
        return
    }
    println!("Number at index {index} is {}", my_arr4[index as usize]);
}

fn learn_tuples() {
        // we can declare a tuple with optional type annotation
        let my_tup: (u32, &str, bool) = (321, "Hello Rustoshi", false);
        println!("{:#?}", my_tup);
    
        // we can destructure individual items
        let (x,y,z) = my_tup;
        println!("The value of y is {}", y);
    
        // we can also use index dot notation
        let isActive = my_tup.2;
        println!("The dev active staus is {}", isActive);
}

fn if_statements() {
    // handling multiple conditions
    println!("Enter your age: ");

    let mut age = String::new();

    io::stdin()
    .read_line(&mut age)
    .expect("Please enter your age");

    let age: i32 = age.trim().parse().expect("Please enter a number for age");

    if age >= 18 && age <= 25 {
        println!("You're a young adult");
    } else if age >= 26 && age <= 35 {
        println!("You are in your prime man!");
    } else if age >= 36 && age <= 45 {
        println!("Man you ain't getting any younger");
    }else if age >= 46 {
        println!("You're too old for this.");
    } else {
        println!("Go home lad");
    }

    // using if in a let statement
    let gender = "Male";
    let read_gender = if gender == "Male" {"He is a male"} else {"He is a female"};
    println!("{}", read_gender);


}

fn learn_loops() {
    
}

fn main() {
    // number types
    // learn_nunmber();

    // tuples
    // learn_tuples();

    // arrays
    // learn_arrays();

    // if statments
    // if_statements();

    // loops


}