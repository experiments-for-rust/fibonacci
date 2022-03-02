use std::io::{stdin,stdout,Write};

fn main(){
    let mut a=String::new();
    loop{
        print!("Input arity: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut a).expect("Error occurred.");
        break;
    }
    let a:i32=guess.trim().parse().expect("Error occurred.");
    
    let mut i=1;
    while i<=a {
        print!("{} ",fnbc(i));
    }
    println!("");
}
fn fbnc(a:i32)->i32{
    if a=1|a=2{
        return 1;
    }
    return fnbc(a-2)+fnbc(a-1);
}