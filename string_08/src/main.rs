fn main() {
    println!("Hello, world!");

    //&str 是在模块std:str,字符串切片
    let lesson="Rex 学习 rust 字符串";

    /*
    字符串对象 在堆上分配 可以在运行时提供字符串值以及响应的操作方法
    String::new() 创建一个新的空字符串，他是静态的
    String::from() 从具体的字符串字面量创建字符串对象

     */

    let s1=String::new();

    println!("s1:{},s1-len:{}",s1,s1.len());


    let s2=String::from("我在学习Rust");

    println!("s2:{},s2-len:{}",s2,s2.len());



    let mut s3=String::new();

    s3.push_str("Rex 学习");
    s3.push('r');
    s3.push('x');
    println!("s3 {}",s3);


    let s4=String::from("要被替换的内容");
    println!("s4 {}",s4);

    let result= s4.replace("要被替换的内容","www.icods.cn");
    println!("result {}",result);

    let s5="从str 变成 String".to_string();
    print!("length {}",s5.len());

    let s6=String::from("要打印的字符串");
    printname(s6.as_str());

    fn printname(name:&str){
        println!("字符串={}",name);
    }


    //去掉头尾空白符 制表符\t 空格 回车\r 换行\n 回车换行\r\n
    let s7="\t我是阿达阿萨\t撒旦撒 \r\n实打实 \r\nsdfsfff\r\n   ";
    println!("s7.len{} ",s7.len());
    println!("s7.trim().len{} ",s7.trim().len());


    let s8="黄瓜、土豆、西红柿";
    for vegetable in s8.split('、'){
        println!("vegetable is {} ",vegetable);
    }

    let s10="黄瓜、土豆、西红柿";
    for ch in s8.chars(){
        println!("ch is {} ",ch);
    }


    let s11="Rex ".to_string();
    let s12="在学习".to_string();
    let result2=s11+&s12;
    println!("result2 {}" ,result2);
}
