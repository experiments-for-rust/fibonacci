use std::io::{stdin,stdout,Write};

fn main(){
    let mut a=String::new();
    loop{
        print!("Input arity: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut a).expect("Error occurred.");
        let a:i32=match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        let mut i=1;
        while i<=a {
            print!("{} ",fbnc(i));
            i+=1;
        }
        println!("");
        break;
    }
}

fn fbnc(a:i32)->i32{
    if a==1||a==2{
        return 1;
    }
    return fbnc(a-2)+fbnc(a-1);
}