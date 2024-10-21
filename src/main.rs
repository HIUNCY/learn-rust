fn main() {
    variable();
    numeric_operator();
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

fn numeric_operator() {
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("The sum of {} and {} is {}", a, b, c);
    let d = a - b;
    println!("The difference of {} and {} is {}", a, b, d);
    let e = a * b;
    println!("The product of {} and {} is {}", a, b, e);
    let f = a / b;
    println!("The quotient of {} and {} is {}", a, b, f);
    let g = a % b;
    println!("The remainder of {} and {} is {}", a, b, g);
}
