use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hello, world!");
    let mut vec = vec!["zzn", "name", "rex"];


    let mut hashmap = HashMap::new();

    hashmap.insert("name", "rex");
    hashmap.insert("age", "24");

    let boo = hashmap.contains_key("name");
    println!("boo {},map.len {}", boo, hashmap.len());


    let mut hashset = HashSet::new();
    hashset.insert("name set");
    hashset.insert("rex set");
    hashset.insert("zzn set");

    println!("{:?}", hashset);
    hashset.get("rex set");

    match hashset.get("rex set") {
        Some(data) => {
            println!("找到了 {}", data)
        }
        None => {
            println!("没有 rex set")
        }
    }
}

