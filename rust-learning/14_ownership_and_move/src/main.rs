fn main() {
    /*
    基本类型所有权 栈
     */
    let a = 100;
    let b = a;
    show_num(a);
    println!("a is {}", a);


    /*
    对象 堆 所有权的转移
     */
    let name_list = vec!["name", "zzn", "rex"];
    show_namelist(name_list);
    // println!("name list {:?}",name_list); //引用类型已经被借用


    /*
对象 堆 所有权的转移
 */
    let name_list2 = vec!["name", "zzn", "rex"];
    let result=move_ownership(name_list2);
    println!("result list {:?}",result);
}

fn show_num(num: i32) {
    println!("num is {}",num);
}

fn show_namelist(namelist: Vec<&str>) {
    println!("namelist is {:?}", namelist);
}


fn move_ownership(namelist:
                  Vec<&str>)->Vec<&str>{
    println!("namelist is {:?}", namelist);
    return namelist;
}

