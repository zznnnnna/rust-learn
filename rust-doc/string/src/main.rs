use std::fmt::format;

fn main() {
    println!("Hello, world!");
    let str = "ddddd";

    let string_a = String::from("aaa");
    let string_b = String::from("bbb");
    let string_c = String::from("ccc");

    let string_a_b = string_a + &string_b;

    let string_a_b_c = format!("{}{}", &string_a_b, string_c);

    println!("string_a_b is {}", string_a_b);
    println!("string_a_b_c is {}", string_a_b_c);
    
    for bytes in string_a_b_c.bytes() {
        println!("{}", bytes)
    }
    
    for char in string_a_b_c.chars() {
        println!("{}", char)
    }
}
