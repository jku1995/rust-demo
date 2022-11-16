use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    println!("guessing a number!");
    println!("pls input your guess");
    let rand_number = rand::thread_rng().gen_range(1..=3);
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("read failed!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(..) => continue,
        };
        match guess.cmp(&rand_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too Greater"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
