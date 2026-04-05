use std::io;

fn left_pyramid(){
    let mut h=String::new();
    println!("Enter height for the pyramid");
    io::stdin().read_line(&mut h).expect("something please");
    let h: u32=h.trim().parse().expect("please be a number");
    let mut c=String::new();
    println!("character to make up the pyramid");
    io::stdin().read_line(&mut c).expect("make sure it's one character");
    let c=c.trim().chars().next().expect("one character only");

    println!(" ");
    
    for i in 1..=h{
        for _j in 1..i{
            print!("{}",c);
        }
        print!("\n")
    }
}

fn right_pyramid(){
    let mut h=String::new();
    println!("Enter height for the pyramid");
    io::stdin().read_line(&mut h).expect("something please");
    let h: u32=h.trim().parse().expect("please be a number");
    let mut c=String::new();
    println!("character to make up the pyramid");
    io::stdin().read_line(&mut c).expect("make sure it's one character");
    let c=c.trim().chars().next().expect("something to be printed");

    println!(" ");

    for i in 1..=h{
        for _j in (i..=h){
            print!(" ");
        }
        for _x in (1..=i){
            print!("{}",c);
        }
        print!("\n");
    }
}

fn center_pyramid(){
    let mut h=String::new();
    println!("Enter height for the pyramid");
    io::stdin().read_line(&mut h).expect("something please");
    let h: u32=h.trim().parse().expect("please be a number");
    let mut c=String::new();
    println!("character to make up the pyramid");
    io::stdin().read_line(&mut c).expect("make sure it's one character");
    let c=c.trim().chars().next().expect("you didn't enter anything");

    println!(" ");

    for i in (1..=h){
        for j in (i..=h).rev(){
            print!(" ");
        }
        for _x in (1..i){
            print!("{} ",c);
        }
        print!("\n");
    }
    
}

fn main() {
    println!("CLI PYRAMID PROGRAMME");
    println!("1.LEFT HAND SIDE PYRAMID");
    println!("2.CENTERED PYRAMID");
    println!("3.RIGHT HAND SIDE PYRAMID");

    let mut choice=String::new();
    println!("ENTER YOUR CHOICE BY NUMBER");

    io::stdin().read_line(&mut choice).expect("say something");

    let choice: u32=choice.trim().parse().expect("expected a number");

    if choice==1{
        left_pyramid();
    }
    else if choice==2{
        center_pyramid();
    }
    else if choice==3{
        right_pyramid();
    }
}
