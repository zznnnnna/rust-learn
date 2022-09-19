fn main() {
    let t = ("zzn", "rex", "john");
    println!("{:?}", t);
    //元组变量.索引数字
    println!("{}", t.0);
    println!("{}", t.1);
    println!("{}", t.2);

    show_tuple(t);

    let (name,name1,name2)=t;
    println!("name is {}",name );
    println!("name1 is {}",name1 );
    println!("name2 is {}",name2);

}

fn show_tuple(t: (&str, &str, &str)) {
    println!("show_tuple is {:?}", t);
}
