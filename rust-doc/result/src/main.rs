
use std::fs::File;

fn main() {
    println!("Hello, world!");
    
    let f=File::open("hello.txt");
    println!("{:?}",f);
    
    let f = match f {
        Ok(success)=>{success},
        Err(error)=>{
            panic!("出错了，错误原因{}",error)}
    };
    
}
