fn main() {
    variable();
    numeric_operator();
    comparison_operator();
    boolean_operator();
    tuple();
    array();
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

fn comparison_operator() {
    let a = 10;
    let b = 20;
    let c = a > b;
    println!("{} is greater than {}? {}",a,b,c);
    let d = a < b;
    println!("{} is less than {}? {}",a,b,d);
    let e = a == b;
    println!("{} is equal to {}? {}",a,b,e);
    let f = a != b;
    println!("{} is not equal to {}? {}",a,b,f);
}

fn boolean_operator() {
    let a = true;
    let b = false;
    let c = a && b;
    println!("{} and {} is {}",a,b,c);
    let d = a || b;
    println!("{} or {} is {}",a,b,d);
    let e = !a;
    println!("not {} is {}",a,e);
}

fn tuple() {
    let tuple = (1, 2.5, false);
    println!("tuple is {:?}", tuple);
    // access tuple element
    println!("{}", tuple.0);
    println!("{}", tuple.1);
    println!("{}", tuple.2);

    // destructuring tuple
    let (a, b, c) = tuple;
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    // tuple.0 = 10; ERROR

    // NOTE: same as variable, the default of tuple is immutable. if u want to change the value of tuple, u need to use mut
}

fn array() {
    let array = [1, 2, 3, 4, 5];
    println!("array is {:?}", array);
    println!("array length is {}", array.len());

    // access array element
    println!("{}", array[0]);
    println!("{}", array[1]);
    println!("{}", array[2]);
    println!("{}", array[3]);
    println!("{}", array[4]);

    // destructuring array
    let [a, b, c, d, e] = array;
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
    println!("{}", e);
    // array[0] = 10; ERROR

    // NOTE: same as variable, the default of array is immutable. if u want to change the value of array, u need to use mut

    // two dimensional array
    let matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];
    println!("matrix is {:?}", matrix);
}
