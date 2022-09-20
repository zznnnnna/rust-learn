fn main() {
    //引用类型变量
    let a=&66;
    match a {
        &a_value=>(println!("a values is {:?}",a_value))


    }

    match *a {
        a_value=>(println!("a values is {:?}",a_value))
    }

    match a {
        ref a_value=>(println!("a values is {:?}",a_value))
    }

    //非引用类型变量
    let b=99;
    match b {
        ref b_value=>(println!("b values is {:?}",b_value))
    }

    let mut b2=100;
    match b2 {
        ref mut  b2_value=>{
            *b2_value=*b2_value*2;//解引用
            println!("b2 values is {:?}",b2_value)   ;
        }
    }

    //解结构体
    struct Person{
        name:String,
        age:i32
    }

    let zzn=Person{
        name:String::from("rex"),
        age:24
    };

    let Person{name,..}=zzn;
    println!("name is {}",name);

}
