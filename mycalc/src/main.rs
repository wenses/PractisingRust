use std::io;

fn myadd(x: u32, y: u32) -> u32{

    return x+y
}

fn mysub(x: u32, y: u32) -> u32{
    return x-y
}

fn myprod(x: u32, y: u32) -> u32{
    return x*y
}

fn mydiv(x: u32, y: u32) -> u32{
    return x/y
}

fn main() {
    println!("TWO VALUE CALCULATOR");
    let mut x=String::new();
    let mut y=String::new();

    println!("Enter the first value: ");
    io::stdin().read_line(&mut x).expect("anything");
    let x: u32=x.trim().parse().expect("a number please");

    println!("Enter the second value: ");
    io::stdin().read_line(&mut y).expect("anything");
    let y: u32=y.trim().parse().expect("a number please");

    println!("YOUR VALUES ARE X={}, Y={} PLEASE CHOOSE OPERATIONS",x,y);
    println!("1.ADDITION, 2.SUBSTRACTION, 3.PRODUCT, 4.DIVISON");
    let mut choice=String::new();
    io::stdin().read_line(&mut choice).expect("anything please");
    let option: u32=choice.trim().parse().expect("choice please :)");

    match option{
        1 => {
            let mut ans: u32=myadd(x, y);
            println!("{}",ans);
            
        }
        2 => {
            let mut ans: u32=mysub(x,y);
            println!("{}",ans);
        }
        3 => {
            let mut ans: u32=myprod(x, y);
            println!("{}",ans);

        }

        4 => {
            let mut ans: u32=mydiv(x,y);
            println!("{}",ans);
        }
        _ => {
            println!("invlaid choices");
        }
    }
    
}
