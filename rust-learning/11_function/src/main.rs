fn main() {
    /*
    函数
    fn 函数名称([参数:基础语法])->返回值{
    //函数代码
    }

 */
    hello();
    println!("name have return {}", get_name());
    // println!("name have not return {}",get_name2() );

    /*
    值传递
     */
    let mut price = 100;
    double_price(price);
    println!("外部的price is {}", price);



    /*
    引用传递
     */
    let mut price2 = 300;
    double_price2(&mut price2);
    println!("外部的price is {}", price2);


    /*
    复合类型传参传递后变量不可再访问
     */
    let name3 = String::from("测试复合类型传递");
    get_name3(name3);
    println!("name3 is {}",name3);
    /*
    error[E0382]: borrow of moved value: `name3`
  --> src/main.rs:35:28
   |
33 |     let name3 = String::from("测试复合类型传递");
   |         ----- move occurs because `name3` has type `String`, which does not implement the `Copy` trait
34 |     get_name3(name3);
   |               ----- value moved here
35 |     println!("name3 is {}",name3);
   |                            ^^^^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

     */

}

fn hello() {
    print!("hello rust!!");
}


fn get_name() -> String {
    return String::from("Im rex");
}

// fn get_name2()->String{
//    return  String::from("Rex ");
// }


fn double_price(mut price: i32) {
    price = price * 2;
    println!("方法内部price is {}", price
    )
}


fn double_price2(price: &mut i32) {
    /*
    * 解引用符
    指向内存位置上存储的变量的值
     */
    *price = *price * 2;
    println!("方法内部price is {}", price
    )
}

fn get_name3(name:String){
    println!("被使用的name ：{}",name)
}