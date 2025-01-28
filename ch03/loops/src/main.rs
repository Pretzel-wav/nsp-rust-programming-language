fn main() {

    // pg 54, Returning Values from Loops
    let mut counter = 0;

    let result =  loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // pg 55, Conditional Loops with while
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
    println!("LIFTOFF!!!");
}
