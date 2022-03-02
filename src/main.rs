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
        print!("{} ",fbnc(i));
        i+=1;
    }
    println!("");
}
fn fbnc(a:i32)->i32{
    if a==1||a==2{
        return 1;
    }
    return fbnc(a-2)+fbnc(a-1);
}