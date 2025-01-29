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
    let y = x; // this works as expected, since integers are on the stack

    let s1 = String::from("hello");
    let s2 = s1; // this copies the *reference* to the data on the heap.
                 // s2 is the pointer/length/capacity of the data on the heap,
                 // but it is not a copy of the data on the heap.
                 // This pointer/length/capacity is what goes on the stack.
                 // So s1 and s2 are two copies of a reference to the same data on the heap.

    // println!("{}", s1); // error, since s1 is now invalid. s2 became the owner of the data on the heap.
}

// pg 65, Ways That Variables and Data Interact: Clone
fn example_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // clone() copies both the reference *and* the data on the heap.
                         // This is more expensive than working with the ownership model!

    println!("s1: {}, s2: {}", s1, s2); // no error, since both variables are still valid.
}

// pg 68, Ownership and Functions
fn example_functions() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to
                                    // still use x afterward
}   // Here, x goes out of scope, then s. But because s's value was moved,
    // nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
}   // Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.
    
fn makes_copy(some_integer: i32) {  // some_integer comes into scope
    println!("{}", some_integer);
}   // Here, some_integer goes out of scope. Nothing special happens.

// pg 69, Return Values and Scope
fn example_return_values() {
    let s1 = gives_ownership();         // gives_ownership moves its return value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back,
                                        // which also moves its return value into s3
}   // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was moved,
    // so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {        // gives_ownership will move its return value
                                        // into the function that calls it
    let some_string = String::from("hello");    //  some_string comes into scope

    some_string                         // some_string is returned and moves out to
                                        // the calling function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope

    a_string // a_string is returned and moves out to the calling function
}

// pg 69, Return Values and Scope (part 2)
fn example_multiple_values() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1); // it's annoying to pass the string back to the calling function like this!

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

// pg 70, References and Borrowing
fn example_borrowing() {
    let s1 = String::from("hello");

    let len = calculate_length_with_reference(&s1); // creates reference to value in s1
    
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length_with_reference(s: &String) -> usize { // s is a reference to a String
    s.len()
}   // Here, s goes out of scope. But because it does not have ownership of what it
    // refers to, nothing happens.

// pg 71, References and Borrowing (part 2)
fn example_change_borrowing() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &str) {
    some_string.push_str(", world"); // error! You can't modify a borrowed value!
}

// pg 72, Mutable References
fn example_mut_references() {
    let mut s = String::from("hello");  // s was immutable; make it mutable

    change(&mut s);                     // pass in a mutable reference
}

fn change(some_string: &mut String) {   // take a mutable reference
    some_string.push_str(", world!");
}

// pg 72, Mutable References (part 2)
fn example_multiple_mut_refs() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; // error! cannot borrow `s` as mutable more than once at a time!
}

// pg 73, Mutable References (part 3)
fn example_brackets_mut_refs() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    }   // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}

// pg 73, Mutable References (part 4)
fn example_multiple_mut_refs_with_ref() {
    let mut s = String::from("hello");

    let r1 = &s;        // no problem
    let r2 = &s;        // no problem
    let r3 = &mut s;    // error! Cannot borrow as mutable because it is also borrowed as immutable!
}

// pg 74, Dangling References
fn example_dangling_refs() {
    let reference_to_nothing = dangle(); // reference obtained, but data went out of scope in function
}

// error! Missing lifetime specifier! The solution is to return s directly, not the reference.
fn dangle() -> &String {
    let s = String::from("hello"); // s comes into scope

    &s                             // reference to s created
}                                  // reference moved back to calling function, then s goes out of scope

// pg 76, The Slice Type
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

