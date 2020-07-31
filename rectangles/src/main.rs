#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn calc_area(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rectangle is {:#?}", rectangle);
    println!("Area of rectangle is {}", calc_area(&rectangle))
}
