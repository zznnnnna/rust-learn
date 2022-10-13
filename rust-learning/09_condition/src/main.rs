fn main() {
    println!("Hello, world!");

    let total:f32=788.00;
    if total>500.00 {
        println!("打八折 {}",total*0.8);
    }else{
        println!("打骨折");
    }


    let code="10010";
    let choose =match code{
        "10010"=>"联通",
        "10086"=>"移动",
        _=>"啥也不是"

    };

    println!("choose is {}",choose);

}
