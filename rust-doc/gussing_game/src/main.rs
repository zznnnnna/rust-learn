use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    //随机数
    println!("这是一个猜数游戏！");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("随机数是： {secret_number}");


    loop {
        let mut number = String::new();

        println!("输入你的猜想");

        io::stdin()
            .read_line(&mut number)
            .expect("输入错误");

        // println!("your guess is {}", number);
        // println!("your guess is {number}");

        // let number: u32 = number.trim().parse().expect("类型转换失败");
        let number: u32 =
            match number.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };


        match number.cmp(&secret_number) {
            Ordering::Less => println!("猜小了"),
            Ordering::Equal => {
                println!("猜对了 即将退出!");
                break;
            }
            Ordering::Greater => println!("猜大了")
        }
    }
}
