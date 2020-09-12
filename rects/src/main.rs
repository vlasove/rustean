#[derive(Debug)]
struct Rectangle {
    width : u32,
    height: u32,
}

impl Rectangle{
    //constructor
    fn new(w : u32, h:u32) -> Rectangle {
        return Rectangle{
            width : w,
            height : h,
        };
    }
    //regular method
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
}

fn area(rect: &Rectangle) -> u32 {
    return rect.height * rect.width;
}
fn main() {
    let rect = Rectangle::new(10,20);
    let a = area(&rect);
    println!("The area of rect func : {}", a);
    println!("The area of rect method: {}", rect.area());
    println!("{:#?}", rect);
}
