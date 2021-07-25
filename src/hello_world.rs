use std::io;

pub fn hello_world(){
    println!("Hello, World");
}

pub fn personal_greeting(){
    println!("What is your name?");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");

    println!("Greetings, {}", name);
}

pub fn discriminitory_greeting(){
    println!("What is your name?");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");

    match name.to_lowercase().trim() {
        "bob" => println!("Hey Bob!"),
        "alice" => println!("Hey Alice!"),
        _ => println!("You're not Bob or Alice...")
    }
    
}