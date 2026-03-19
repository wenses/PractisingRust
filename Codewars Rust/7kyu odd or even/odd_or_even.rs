fn odd_or_even(numbers: Vec<i32>) -> String {
    let mut sum: i32 = 0;
    for number in numbers.iter(){
        sum+=number
    }
    if sum%2==0{
    "even".to_string()
        }
    else{
        "odd".to_string()
    }
}