fn main() {
    let width1 = 30;
    let height1 = 50;

    let rect1 = (30, 50);

    let rect2 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area_by_tuple(rect1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area_by_struct(&rect2)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );

    // println!("rect2 is {}", rect2);  // error! Rectangle doesn't implement 'std::fmt::Display'
    println!("rect2 is {:?}", rect2);   // use Debug instead via {:?} (also derive Debug on struct)
}

// the evidence something is wrong here is "The area function is supposed to calculate
// the area of one rectangle, but the function we wrote has two parameters."
fn area(width: u32, height: u32) -> u32 {
    width * height
}

// here, we pass in one value for one rectangle by using a tuple. However, because tuples
// don't name their elements, we don't know which is width and which is height.
fn area_by_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// creating the Rectangle struct allows us to structure the data and also name the parts
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_by_struct(rect: &Rectangle) -> u32 { // reference because we're just calculating on it
    rect.width * rect.height
}

// creating the method on the struct itself ties the behavior more closely to the struct,
// which is useful if the function won't work on any other type.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}