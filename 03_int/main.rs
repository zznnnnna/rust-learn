fn main(){

    //有符号类型 signed可以存储正数和负数
    //无符号类型 usigned只能存储整数
    //按存储空间来说 分为 1、2、4、8、16字节 1字节=8位
let price=100;
let price2:u32=200;
let price3:i32=-300;
let price4:isize=400;
let price5:usize=500;


println!("prince IS {}",price);
println!("prince2 IS {}",price2);
println!("prince3 IS {}",price3);
println!("prince4 IS {}",price4);
println!("prince5 IS {}",price5);

// let price6:i8=128; //-128 - 127

}