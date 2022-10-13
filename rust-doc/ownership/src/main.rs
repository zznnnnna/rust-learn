fn main() {
    // println!("Hello, world!");
    let x: i32 = 5;
    let y = x;
    println!("{x} {y}");

    let str1="safasf";
    let str2=str1;
    println!("{str1} {str2}");

    let str3=String::from("sadsfdgfasds");
    // let str4=str3;
    // println!("{str3} {str4}");
    heap(&str3);
    println!("outer str3 is {str3}");


    let mut str =String::from("sadsadsad");
    mut_hep(&mut str);
    println!("mut str is {str}")


}

fn heap(str:&String){
    println!("inner str3 is {str}");
}

fn mut_hep( str: &mut String){
    str.push_str(", is changed");
}