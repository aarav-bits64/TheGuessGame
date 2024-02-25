use rand::Rng;
use std::io;

fn main() {
    let mut rnum = rand::thread_rng().gen_range(1..=100);
   

    loop {
        let mut answer = String::new();
    println!("Enter a number between 1 and 100: ");
    io::stdin().read_line(&mut answer).expect("error reading line.");

    let mut num:i32  = answer.trim().parse().expect("Please enter an integer.");

    if num>rnum{
        println!("Too big!");
    } else if num<rnum{
        println!("Too small!");
    } else if num==rnum{
        println!("You Win!");
        return;
    }
    }
}
