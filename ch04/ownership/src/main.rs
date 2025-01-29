// pg 62, Variable Scope
fn main() {             // s is not valid here; it's not yet declared
    let s = "hello";    // s is valid from this point forward

    // do stuff with s
}                       // this scope is now over, and s is no longer valid

// pg 63, Memory and Allocation
fn via_string_from() {
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}                                  // this scope is now over, and s is no longer valid

// pg 64, Ways That Variables and Data Interact: Move
fn example_move() {
    let x = 5;
    let y = x;
}