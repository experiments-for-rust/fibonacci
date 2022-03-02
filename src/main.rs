use std::io::{stdin,stdout,Write};

fn main() {
    let mut a = String::new();
    loop{
        print!("Input arity: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut a).expect("Error occurred.");
        break;
    }
    let a:i32 = guess.trim().parse().expect("Error occurred.");
}