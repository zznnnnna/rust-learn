fn main() {
    println!("Hello, world!");

    self_function(32, 'z');

    code_state();

    let plus_five=plus_one(5);
    println!("plus five is {plus_five}");
}


fn self_function(num: i32, lable: char) {
    println!("num is {num}, label is {lable}");
}

fn code_state() {
    let x = {
        let y = 299;
        y + 1
    };

    println!("y is {x}");
}


fn plus_one(x: i32) -> i32 {
    x + 1
}