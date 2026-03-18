use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("WELCOME TO MY GUESSING GAME");

    let x=rand::thread_rng().gen_range(1..=100);
    println!("THE SECRET NUMBER IS {}",x);

    loop{
        let mut guess=String::new();
        println!("Enter your guess: ");
        
        io::stdin().read_line(&mut guess).expect("error reading line");

        //converting to unsinged integer 32 (small poisitive number)
        let guess: u32=guess.trim().parse().expect("Please enter numbers");

        match guess.cmp(&x){
            Ordering::Less => println!("try bigger"),
            Ordering::Greater => println!("Try smaller"),
            Ordering::Equal => {
                println!("YOU WIN!");
                break;
            }
        }

    }
}
