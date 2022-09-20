fn reveices_closure<F>(closure: F)
    where F: Fn(i32, i32) -> i32,
{
    let result = closure(2, 3);
    println!("接收闭包 is {}", result);
}


fn reveices_closure2<F>(closure: F)
    where F: Fn(i32) -> i32, {
    let result = closure(2);
    println!("接收闭包 is {}", result);
}

fn return_closure() -> impl Fn(i32) -> i32 {
        return |x| x+6;
}

fn main() {
    //1、接收闭包
    let add = |x, y| x + y;
    reveices_closure(add);

    //2、接收闭包
    let y2 = 2;
    // let add_y = |x| x + y2;
    reveices_closure2(|x| x + y2);

    //3、返回闭包
    let result=return_closure();
    print!("返回闭包=>{}",result(1));

}



