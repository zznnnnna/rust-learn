fn main() {
    // println!("Hello, world!");

    let str = String::from("hello world");

    let str_slice = &str[1..4];

    let str_tail = &str[2..];

    let str_head = &str[..2];


    println!("str_slice is {str_slice}, str_tail  is {str_tail},  str_head is {str_head}");


    let a = [1, 2, 3, 4, 5];
    let a_alice = &a[1..3];
    assert_eq!(a_alice,&[2,3]);

    let world=String::from("hello world");
    let first_w=first_world(&world);
    println!("first_W is {first_w}");
}


fn first_world(str: &String) -> &str {
    let bytes = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("item is {item}");

            return &str[..i];
        }
        println!("item is {item}")
    }
    &str[..]
}