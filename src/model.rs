pub struct User {
    first_name: String,
    last_name: String,
    username: String,
    email: String,
    age: u8,
}
impl User {
    pub fn new(first_name: String, last_name: String, username: String, email: String, age: u8) -> Self {
        User {
            first_name,
            last_name,
            username,
            email,
            age,
        }
    }
    pub fn say_hello(&self) {
        println!("Hello {}!", self.username);
    }
}
