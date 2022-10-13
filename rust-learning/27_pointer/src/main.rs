use std::ops::Deref;

fn main() {
    let a = 6;
    let b = Box::new(a);//box 指针装箱 hap堆  stack栈
    println!("{}", a);

    println!("{}", *b);//* 解引用符


    struct CustomBox<T> {
        value: T,
    }

    impl<T> CustomBox<T> {
        fn new(v: T) -> CustomBox<T> {
           return  CustomBox { value: v };
        }
    }

    impl<T> Deref for CustomBox<T> {
        type Target = T;
        fn deref(&self) -> &T {
           return  &self.value;
        }
    }
    impl<T> Drop for CustomBox<T>{
        fn drop(&mut self){
            //drop 结构体超出作用域会执行drop方法
            println!("被drop() 了");
        }
    }

    let x=666;
    let y=CustomBox::new(x);
    println!("y is {}",*y);//解引用
}
