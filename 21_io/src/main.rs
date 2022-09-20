use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    let result=stdin().read_line(&mut input).unwrap();
    println!("input is {}",input);
    println!("result 字节数 is {}",result);

    let result2=stdout().write("wohhhhh".as_bytes()).unwrap();
    println!("result2 is {}",result2);
}
