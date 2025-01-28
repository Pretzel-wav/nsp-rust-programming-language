fn main() {
    // pg 50, if Expressions
    let number = 8;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("{} is not divisible by 4, 3, or 2", number);
    }

    // pg 52, Using if in a let Statement
    let condition = true;
    let number = if condition {
        5
    } else {
        6 // will generate error if both blocks are not the type declared by let number
    };

    println!("The value of number is: {}", number)
}
