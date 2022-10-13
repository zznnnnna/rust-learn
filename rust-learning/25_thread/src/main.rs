use std::thread;
use std::time::Duration;

fn main() {
//主线程
    let handler = thread::spawn(|| {
        for i in 1..10 {
            println!("子线程{}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
// 子线程
    for i in 1..5 {
        println!("主线程 {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    //子线程结束完主线程再结束
    handler.join().unwrap();//适用join方法 把子线程加入主线程等待队列
}
