fn main() {
    // pg 32, Variables and Mutability
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // pg 34, Shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    // pg 40, Compound Types
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup; // underscores indicate planned unused variables (and suppress warnings about them)
    println!("The value of y is: {}", y);
}
