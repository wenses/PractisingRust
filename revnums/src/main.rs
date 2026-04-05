fn f2(){
    let mut x: u64=102123;

    let x=x.to_string();

    let mut xchars:Vec<char>=x.chars().collect();

    xchars.sort();
    xchars.reverse();

    let mut xfn: String=xchars.into_iter().collect();

    println!("{}",xfn)

}
fn main() {
    /*let mut digits: u64=1021;
    let mut es=String::new();

    let digits=digits.to_string();

    for c in digits.chars().rev(){
        es.push(c);
    }
    println!("{}",es);*/

    f2();
}
