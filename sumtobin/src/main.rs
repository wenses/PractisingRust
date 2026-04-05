fn sum2bin(a: u32, b: u32) -> String{

    let mut c: u32 = a+b;
    let binary_str=format!("{:b}",c);

    binary_str
}
fn main() {
    //this program takes the sum of two decimal numbers and gives the output in a binary string
    println!("{}",sum2bin(3,4));
}
