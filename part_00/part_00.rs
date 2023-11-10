pub fn double_number(number : i32) -> i32
{
    return number * 2;
}
pub fn main() {
    let mut number = 2;
    println!("number = {number}");
    number = double_number(number);
    println!("number = {number}");
    number = double_number(number);
    println!("number = {number}");
    println!{"hello world!"};
}
