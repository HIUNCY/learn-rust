fn main() {
    variable();
}

fn variable() {
    // dafault variable in rust is immutable that means it can't be changed
    let name = "Zainul";
    // name = "Ajay"; ERROR
    println!("Hello, {}!", name);

    // mut keywoard in rust is mutable that means it can be changed
    let mut name2 = "Zainul";
    name2 = "Ajay";
    println!("Hello, {}!", name2);
}
