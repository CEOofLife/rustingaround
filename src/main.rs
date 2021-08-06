use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    say();
    let secret = rand::thread_rng().gen_range(1..101); 
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");
    
    println!("Your number is {}", guess);
    let guess: u32 = guess.trim().parse().expect("Please type a num!");
    match guess.cmp(&secret){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win!"),
    }


}
fn say() {
    println!("Guessing game!");
    println!("Put in your number!");
       
}
