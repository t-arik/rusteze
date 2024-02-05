#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let area = rect1.width * rect1.height;
    println!("rect1 area is {area}");
    println!("rect1 is {:?}", rect1);
}
