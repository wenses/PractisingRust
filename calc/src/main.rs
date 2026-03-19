use std::io;

fn add(){
    let mut x=String::new();
    let mut y=String::new();
    println!("Enter first number");
    io::stdin().read_line(&mut x).expect("error reading line");
    println!("Enter second number");
    io::stdin().read_line(&mut y).expect("error readling line");

    let x: u32=x.trim().parse().expect("enter numbers please");
    let y: u32=y.trim().parse().expect("enter numbers please");

    let ans: u32=x+y;

    println!("The answer is {}",ans);

}

fn sub(){
    let mut x=String::new();
    let mut y=String::new();
    println!("Enter first number");
    io::stdin().read_line(&mut x).expect("error reading line");
    println!("Enter second number");
    io::stdin().read_line(&mut y).expect("error readling line");

    let x: i32=x.trim().parse().expect("enter numbers please");
    let y: i32=y.trim().parse().expect("enter numbers please");

    let ans: i32=x-y;

    println!("The answer is {}",ans);

}

fn prod(){
    let mut x=String::new();
    let mut y=String::new();
    println!("Enter first number");
    io::stdin().read_line(&mut x).expect("error reading line");
    println!("Enter second number");
    io::stdin().read_line(&mut y).expect("error readling line");

    let x: u32=x.trim().parse().expect("enter numbers please");
    let y: u32=y.trim().parse().expect("enter numbers please");

    let ans: u32=x*y;

    println!("The answer is {}",ans);

}


fn main() {
    println!("WELCOME TO MY CALCULATOR");
    println!("1. ADDITION");
    println!("2. SUBSTRACTION");
    println!("3.MULTIPLICATION")
    let mut choice=String::new();
    println!("Choose choice by number from menu: ");

    io::stdin().read_line(&mut choice).expect("error reading line");

    let choice: u32=choice.trim().parse().expect("Enter number in the menu please");

    if choice==1{
        add();
    }
    else if choice==2{
        sub();
    }
    else if choice==3{
        prod();
    }

}
