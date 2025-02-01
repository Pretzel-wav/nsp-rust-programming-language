// base enum, requires IpAddr struct
enum IpAddrKind {
    v4,
    v6
}

// data can be added directly to the enum
enum IpAddr {
    v4(String),
    v6(String),
}

// each type can have a different form
enum IpAddr2 {
    v4(u8, u8, u8, u8),
    v6(String),
}

fn route(ip: IpAddr) -> String {
    match ip {
        IpAddr::v4(_) => String::from("Route like 4"),  // the (_) is because the type is defined as v4(String),
        IpAddr::v6(_)=> String::from("Route like 6"),   // making it a tuple type with one element. The underscore says the value is not important.
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}


fn main() {
    let home = IpAddr::v4(String::from("127.0.0.1"));
    let home2 = IpAddr2::v4(127, 0, 0, 1);
    let loopback = IpAddr::v6(String::from("::1"));

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None; // Type here is Option!

    let x: i8 = 5;
    let y: Option<i32> = Some(5);
    // let sum = x + y; // error! cannot add Option<i8> to i8

    println!("{}", route(home));
    println!("{}", route(loopback));

    let penny = Coin::Penny;
    let quarter = Coin::Quarter(UsState::Alabama);

    println!("The penny is worth {} cents.", value_in_cents(penny));
    println!("The quarter is worth {} cents.", value_in_cents(quarter));

    let one_more = plus_one(y);
    let none = plus_one(None);
    println!("{:?}", one_more);
    println!("{:?}", none);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => println!("something else"), // underscore is basically "else"
    }

    // a match statement with only one relevant arm
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // shorthand for above code
    if let Some(3) = some_u8_value {    
        println!("three");
    } else { // optional else clause
        println!("something else");
    }
}
