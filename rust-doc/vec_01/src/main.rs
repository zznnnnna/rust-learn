fn main() {
    // println!("Hello, world!");
    let v = vec![100, 200, 300];
    let x = &v[0];
    println!("{x}");

    for i in &v {
        println!("{i}")
    }

    let mut v2 = vec![200];

    let x2 = &v2[0];
    let x3 = v2.get(0);
    // let x4 = v2.get(100);


    // v2.push(100);  //这个加了 前一行会报错 ，塞入数据时 vec会重新分配内容

    println!("{} {:?} ", x2, x3);


    let result = match v2.get(0) {
        None => { println!("什么也没获取到") }
        Some(res) => { print!("获取到的是{}", res) }
    };


    let mut manyVec = Vec::new();
    manyVec.push(ManyType::Int(23));
    manyVec.push(ManyType::Name(String::from("如何使用vec存储不同类型的数据")));

}

enum ManyType {
    Int(i32),
    Name(String),
}