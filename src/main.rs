use std::io::{self, Write};
use std::env;
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let mut directory: HashMap<String, Vec<String>> = HashMap::new();
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[0]);
    let program_name = path.file_name().unwrap().to_str().unwrap();
    println!("Welcome to {}!", program_name);

    print_help();
    loop {
        print!("> ");
        io::stdout().flush().expect("Error flushing stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading input");
        let command = input.split_whitespace().next();
        match command {
            Some("add") => add_command(&mut directory, &input),
            Some("remove") => remove_command(&mut directory, &input),
            Some("list") => list_command(&mut directory, &input),
            Some("help") => print_help(),
            Some("exit") => break,
            _ => println!("Unknown command: {}", input),
        }
    }
}

fn print_help() {
    println!("----------------------------------------------------------------------------------");
    println!("help                               Print this help message");
    println!("add [name] to [department]         Add user [name] to department [department]");
    println!("remove [name] from [department]    Remove user [name] from Department [department]");
    println!("list people in [department]        List all people in department [department]");
    println!("list prople                        List all people");
    println!("exit                               Exit the program");
    println!("----------------------------------------------------------------------------------");
    println!();
}

fn list_command(directory: &mut HashMap<String, Vec<String>>, input: &str) {
    println!("Listing {}", input);
}

fn remove_command(directory: &mut HashMap<String, Vec<String>>, input: &str) {
    // split input into words
    let words: Vec<&str> = input.split_whitespace().collect();
    // Expect remove [name] from [department] where name and apartment are single words
    if words.len() != 4 {
        println!("Invalid command: {}", input);
        return;
    }

    //get name
    let name = words[1];

    // get Department
    let department = words[3];

    // remove name from department
    let people = directory.entry(department.to_string()).or_insert(Vec::new());
    let index = people.iter().position(|x| *x == name);
    if let Some(i) = index {
        people.remove(i);
        println!("Removed {} from {}", name, department);
        return;
    } 
    println!("{} is not in {}", name, department);
}

fn add_command(directory: &mut HashMap<String, Vec<String>>, input: &str) {
    // split input into words
    let words: Vec<&str> = input.split_whitespace().collect();
    // Expect add [name] to [department] where name and apartment are single words
    if words.len() != 4 {
        println!("Invalid command: {}", input);
        return;
    }

    //get name
    let name = words[1];

    // get department
    let department = words[3];

    // add name to Department
    let people = directory.entry(department.to_string()).or_insert(Vec::new());
    people.push(name.to_string());
    println!("Added {} to {}", name, department);
}
