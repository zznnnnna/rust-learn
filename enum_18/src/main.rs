#[derive(Debug)]

enum NameEnum{
    REX,
    JOHN
}


enum PersonEnum{
    Name(String),
    Age(i32)
}

fn match_personEnum(p:PersonEnum){
     match p {
        PersonEnum::Age(i32)=>{
            println!("是年龄");
        },
         PersonEnum::Name(i32)=>{
             println!("是名字");
         },
     }

}

fn main() {
    let isEnum=NameEnum::REX;
    println!("isEnum is {:?}",isEnum);

    let nameEnum=PersonEnum::Name(String::from("rex"));
    match_personEnum(nameEnum);

}
