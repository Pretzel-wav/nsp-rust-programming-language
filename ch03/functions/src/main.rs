fn main() {
    println!("Hello, world!");

    another_function(5, 50);

    let five = five();
    println!("The value of five is {}", five);

    let one_added = add_one(five);
    println!("Adding one results in {}", one_added);
}

fn another_function(x: i32, y: i32) {
    println!("The value of (x,y) is: ({}, {})", x, y);
}

// pg 47, Functions with Return Values
fn five() -> i32 {
    5
}

fn add_one(x: i32) -> i32 {
    x + 1
}