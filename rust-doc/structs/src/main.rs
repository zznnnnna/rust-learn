fn main() {
    println!("Hello, world!");
    let user=user{
        name: "zzn".to_string(),
        age: 0,
        isNb: false,
        email: "".to_string()
    };

    println!("user name is {}",user.name);


    let user2=user{
        email:"z@icods.cn".to_string(),
        ..user

    };

    println!("user2 email is {}",user2.email);


}
struct user{
    name:String,
    age:i32,
    isNb:bool,
    email:String
}

fn build_user(email:String,age:i32)->user
{
    user{
        name: "".to_string(),
        age: 0,
        isNb: false,
        email
    }

}