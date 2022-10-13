
fn main() {
    println!("Hello, world!");
    /*
    const 常量名称：数据类型=值
    常量名称通常是大写字母
    常量可以在任何地方定义 常量只是一个符号 编译时替换为具体的值
    static 具有 ’static生命周期的 可以是可变的变量（static mut关键字）
     */

    const PI:f64=3.124214214;
    println!("{}",PI);

    //变量的隐藏
    let name="zzn";
    let name="rex";
    println!("{}",name);


    //变量改变类型
    let price=200;
    let price="rex";
    println!("{}",price);


    //常量不可以被隐藏和重复定义
    // const DISCOUNT:f64=0.8;
    const DISCOUNT:f64=0.7;
    println!("{}",DISCOUNT);


    static BOOK:&'static str="Rex ";
    println!("BOOK {}",BOOK)
}
