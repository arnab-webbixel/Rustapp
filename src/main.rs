macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}
macro_rules! create_user {
    ($name:ident, $age:expr) => {
        struct User {
            name: String,
            age: u32,
        }
        impl User {
            fn new() -> User {
                User {
                    name: String::from(stringify!($name)),
                    age: $age,
                }
            }
            fn info(&self) {
                println!("User: {}, Age: {}", self.name, self.age);
            }
        }
    };
}



fn say_hello(){
    println!("Hello, function!");
}
fn main(){
    say_hello();
    create_function!(foo);
    create_function!(bar);
    foo();
    bar();
    create_user!(Alice, 30);
    let user = User::new();
    user.info();
}
