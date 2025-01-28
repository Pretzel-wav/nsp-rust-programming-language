fn main() {
    println!("Hello, world!");

    another_function(5, 50);
}

fn another_function(x: i32, y: i32) {
    println!("The value of (x,y) is: ({}, {})", x, y);
}