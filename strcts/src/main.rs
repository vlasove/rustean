#[derive(Debug)]
struct User {
    username : String ,
    email : String, 
    sign_in_count : u64,
    active: bool,
}

fn print_user(u : &User) {
    println!("Name : {}", u.username);
    println!("Email: {}", u.email);
    println!("Sign ins: {}", u.sign_in_count);
    println!("Active: {}", u.active);
}

fn build_user(name: String, email: String) -> User {
    return User {
        username : name,
        sign_in_count : 1,
        active : true, 
        email : email,
    };
}

fn main() {
    let mut user = build_user("Bob".to_string(), "bob@ya.ru".to_string());
    print_user(&user);
    user.email = String::from("new_bob@ya.ru");
    println!("After change");
    print_user(&user);
}
