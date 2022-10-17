use std::collections::HashMap;

fn main() {
    // println!("Hello, world!");
    let mut m = HashMap::new();
    m.insert("name", "rust");
    let field_name = "age";
    let field_value = "1.8";
    m.insert(field_name, field_value);
    println!("m is {:?}", m);

    let field_name2 = String::from("version");
    let field_value2 = String::from("1.60.0");
    m.insert(&field_name2, &field_value2);
    println!("m is {:?}", m);


    let mut m2 = HashMap::new();
    m2.insert("name", "rust");
    let name = match m2.get("name") {
        None => { "" }
        Some(name) => { name }
    };
    println!("{name}");

    for (key, value) in m2 {
        println!("{}   {}", key, value)
    }


    let mut m3 = HashMap::new();
    m3.insert("name", "rust");
    m3.entry("name").or_insert("java");//只有键不存在时 才放进去
    m3.insert("age", "1.60.0");
    m3.insert("age", "1.61.0");

    println!("{:?}", m3);


    let mut m4 = HashMap::new();
    let text = "hello rust hello c++";
    for te in text.split_whitespace() {
        let count = m4.entry(te).or_insert(0);
        *count += 1;//* 星号 解引用
    }
    
    println!("m4 is {:?}",m4);
}

