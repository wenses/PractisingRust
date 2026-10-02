use std::io;

fn main() {
    println!("What is your name");
    let mut name=String::new();

    io::stdin().read_line(&mut name).expect("a name");

    println!("Shalom {}",name);

    println!("What is your age: ");
    let mut age=String::new();
    io::stdin().read_line(&mut age).expect("anything please");
    let age: u32=age.trim().parse().expect("a positive number");

    println!("the double of your age is {}",age*2);

}
