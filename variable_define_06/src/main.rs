use std::path::Component::Prefix;

fn main() {
    println!("Hello, world!");
    /*
    定义变量的格式
    let 变量名=值；不指定变量类型
    let 变量名:变量类型=值；指定变量类型

    变量就是给某一个内存地址起名
    1.可以是字母数字下划线
    2.数字不能开头
    3.变量名区分大小写

    可变变量
    mut关键字 mutable缩写
    let mut 变量名=值;
    let mut 变量名：数据类型=值；


    可变变量

    */

    let study="";
    println!("study {}",study);

    let mut price=188;
    price=288;
    println!("{}",price)
}
