fn main() {
    variable();
    numeric_operator();
    comparison_operator();
    boolean_operator();
    tuple();
    array();
    conditional_statement();
    looping();
    say_hello("Zainul", "Kamal");
    print_text("Ajay", 3);
    let result = factorial(5);
    println!("Factorial result is {}", result);
    struct_example();
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

fn conditional_statement() {
    let score = 65;

    let result = if score >= 75 {
        "Congratulations!"
    } else if score >= 60 && score < 75 {
        "Average!"
    } else {
        "Try again!"
    };

    println!("{}", result);
}

fn looping() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }

        println!("Counter: {}", counter);
    }

    counter = 0;
    while counter < 10 {
        if counter % 2 == 0 {
            println!("Counter: {}", counter);
        }
        counter += 1;
    }

    let arr = ["a", "b", "c", "d"];
    for item in arr {
        println!("Value: {}", item);
    }
}

fn say_hello(first_name : &str, last_name : &str) {
    println!("Hello, {} {}!", first_name, last_name);
}

fn print_text(text : &str, times : u32) {
    if times == 0 {
        return;
    } else {
        println!("{}", text);
    }
    print_text(text, times - 1);
}

fn factorial(n : u32) -> u32 {
    if n <= 1 {
        return 1;
    }
    return n * factorial(n - 1);
}

struct Person {
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u8,
}

fn struct_example() {
    let person = Person {
        first_name: String::from("Muhamad"),
        middle_name: String::from("Zainul"),
        last_name: String::from("Kamal"),
        age: 22,
    };
    println!("Person name: {}", person.first_name);
    println!("Person name: {}", person.middle_name);
    println!("Person name: {}", person.last_name);
    println!("Person name: {}", person.age);

    let person2 = Person {
        first_name: person.first_name.clone(),
        middle_name: String::from("Zainul"),
        last_name: person.last_name.clone(),
        ..person
    };
    println!("Person name: {}", person2.first_name);
    println!("Person name: {}", person2.middle_name);
    println!("Person name: {}", person2.last_name);
    println!("Person name: {}", person2.age);
}
