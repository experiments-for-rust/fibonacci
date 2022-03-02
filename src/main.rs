use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        print!("Input arity: ");
        stdout().flush().unwrap();

        let mut a = String::new();
        stdin().read_line(&mut a).expect("error");
        
        let a: u64 = match a.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Eroor occurred.");
                continue;
            },
        };

        if a == 0 {
            println!("Eroor occurred.");
            continue;
        }

        let mut i = 1;
        while i <= a {
            print!("{} ", fbnc(i));
            i += 1;
        }
        println!("");
        break;
    }
}

fn fbnc(a: u64) -> u64 {
    if a == 1 || a == 2 {
        return 1;
    }
    return fbnc(a - 2) + fbnc(a - 1);
}
