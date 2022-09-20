fn main() {

    let mut v=Vec::new();

    v.push("zzn");
    v.push("name");
    v.push("rex");


    println!("len: {:?}",v.len());
    println!("len: {}",v.len());

    let v2=&v[1..3];
    println!("v2 is {:?}",v2);
    println!("v is {:?}",v);

    //可变的切片 引用
    println!("===============");
    println!("v is {:?}",v);
    modify_slice(&mut v[1..3]);
    println!("v is {:?}",v);


}

fn modify_slice( slice:&mut [&str]){
    slice[0]="哈哈哈";
}


