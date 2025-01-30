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
        area_by_struct(rect2)
    );

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
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_by_struct(rect: Rectangle) -> u32 {
    rect.width * rect.height
}