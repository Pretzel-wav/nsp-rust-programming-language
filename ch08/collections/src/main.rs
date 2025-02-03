fn main() {
    let new_v: Vec<i32> = Vec::new();   // new vector of i32 values
    let mut v = vec![1,2,3];                // vector initialized with three values

    v.push(4);  // adding values to vector
    v.push(5);
    v.push(6);
    v.push(7);

    // accessing via [] (returns a reference)
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // accessing via .get() (returns Option<&T>)
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("No third element"),
    }

    // let does_not_exist = &v[100];    // panic! index out of range
    let does_not_exist = v.get(100);    // returns None

    let first = &v[0];

    v.push(6);

    // println!("The first element is {}", first); // error! v.push() is mutable borrow, so this mutable borrow cannot happen

    // how to loop with immutable borrow
    for i in &v {
        println!("{}", i);
    }

    // how to loop with mutable borrow
    for i in &mut v {
        *i += 50; // * is dereference operator
        println!("{}", i);
    }

}