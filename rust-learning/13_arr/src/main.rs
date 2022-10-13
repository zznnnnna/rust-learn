fn main() {
    /*
    数组
     */
    let name_list = ["zzn", "rex"];

    println!("{:?}", name_list);

    print!("{}", name_list.len());

    for item in name_list {
        println!("item is {}", item);
    }

    for item in name_list.iter() {
        println!("item is {}", item);
    }

    show_arr(name_list);


    let mut name_list2 = ["zzz", "xxx"];
    mut_arr(&mut name_list2);
    println!("{:?}", name_list2);
}

fn show_arr(arr: [&str; 2]) {
    for item in 0..arr.len() {
        println!("arr[item] is  {}", arr[item]);
    }
}

fn mut_arr(arr: &mut [&str; 2]) {
    for item in 0..arr.len() {
        arr[item] = "";
    }
}