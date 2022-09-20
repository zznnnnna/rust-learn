/*
struct 结构体名称{
字段名称:字段类型
}


let 参数=构造体名称{
    字段名称：字段值
}
 */
#[derive(Debug)]
struct Person {
    //结构体
    name: String,
    age: i32,
    sex: String,
}

impl Person {
    //方法的定义
    fn get_age(&self) -> i32 {
        return self.age;
    }
}


fn show_person(p: Person) {
    println!("{:?}", p);
}

fn main() {
    let mut zzn = Person {
        name: String::from("rex"),
        age: 22,
        sex: String::from("男"),
    };

    println!("{:?}", zzn);

    zzn.age = 24;
    println!("{:?}", zzn);

    // show_person(zzn);

    print!("zzn age is {}", zzn.get_age());


    let girl=("girl",22,"女" );
    let (name,age,sex)=girl;
    println!("name is  {}",name)
}
