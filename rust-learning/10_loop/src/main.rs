fn main() {
    println!("Hello, world!");

    /*
    for循环
    for 临时变量 左区间..右区间{
    //
    }
     */

    for num in 1..5 {
        println!("num is {}", num);
    }
    // iter() 普通迭代器
    let name_list = vec![
        "rex", "name", "zzn",
    ];

    for name in name_list.iter() {
        match name {
            &"zzn" => println!("zhao ze nan"),
            _ => println!("name is {}", name),
        }
    }

    //会被消耗的迭代器 into_iter

    let name_list2 = vec![
        "rex", "name", "zzn",
    ];

    for name in name_list2.into_iter() {
        match name {
            "zzn" => println!("zhao ze nan"),
            _ => println!("name is {}", name),
        }
    }

    // println!("name_list2 {:?}",name_list2)

    //可变的迭代器 iter_mut
    let mut name_list3 = vec![
        "rex", "name", "zzn",
    ];

    for name in name_list3.iter_mut() {
        *name = match name {
            &mut "zzn" => { "我是zzn 我被改变了" }
            _ => *name,
        }
    }
    println!("name_list3 is {:?}", name_list3);


    /*
    while 循环
     */

    let mut num = 1;
    while num < 20 {
        println!("num now is {}", num);
        num = num * 2;
    }

    /*
    loop循环
     */
    let mut num2=2;
    loop {
        if num2>20 {
            break;
        }
        println!("num2 is {}",num2);
        num2=num2*2;
    }
}
