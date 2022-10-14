fn main() {
    println!("Hello, world!");

    let x = Some(2);
    match x {
        Some(i) => Some(i + 1),
        None => None
    };

    let y = 9;
    match y {
        8 => println!("8"),
        9 => println!("8"),
        _ => println!("hhh {y}")
    };


    let some=Some(245);
    if let Some(number)=some {
        println!("number is  {number}")
    }
}
