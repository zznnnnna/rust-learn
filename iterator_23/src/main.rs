fn main() {
    let v=vec!["zzn","rex","name"];

    let mut it=v.iter();

    println!("{:?}",it.next());

    println!("{:?}",it.next());

    for item in v.iter() {
        println!("item {}",item);
    }


}
