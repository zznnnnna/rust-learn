fn main() {
    println!("Hello, world!");
    let rec = Rectangle {
        width: 2,
        height: 5,
    };
    let area = rec.area();
    println!("{area}")
}

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.height * self.width
    }
}