/*
同步写法 基础
 */

// fn do3() {
//     for i in 1..5 {
//         println!("do3  {}", i);
//     }
// }
//
// fn do4() {
//     for i in 1..5 {
//         println!("do4  {}", i);
//     }
// }
//
// fn main() {
//     do3();
//     do4();
// }


/*
异步写法01
 */
// use std::thread::{sleep, spawn};
// use std::time::Duration;
//
// fn do3() {
//     for i in 1..5 {
//         println!("do3  {}", i);
//         sleep(Duration::from_millis(500));
//
//     }
// }
//
// fn do4() {
//     for i in 1..5 {
//         println!("do4  {}", i);
//         sleep(Duration::from_millis(500));
//
//     }
// }
//
// fn main() {
//     let do3_spawn=spawn(do3);
//     let do4_spawn=spawn(do4);
//
//     do3_spawn.join().unwrap();
//     do4_spawn.join().unwrap();
// }



/*
异步写法02
 */
use std::thread::{sleep, spawn};
use std::time::Duration;

fn do3() {
    for i in 1..5 {
        println!("do3  {}", i);
        sleep(Duration::from_millis(500));

    }
}

fn do4() {
    for i in 1..5 {
        println!("do4  {}", i);
        sleep(Duration::from_millis(500));

    }
}

fn main() {
    let do3_spawn=spawn(do3);
    let do4_spawn=spawn(do4);

    do3_spawn.join().unwrap();
    do4_spawn.join().unwrap();
}