fn main() {

    //variable
    let x = 5;
    let mut mut_x = 6;
    mut_x=7;
    println!("mux_x is {mut_x}");

    //const
    const IM_CONST: u32 = 33;
    println!("IM_CONST x is {IM_CONST}");


    {
        let x = x + 1;
        println!("in x is {x}");
    }

    println!("out x is {x}");

    //bool
    let t=true;
    let f:bool=false;

    //char
    let c='z';

    //tup
    let tp:(i32,f64,char)=(200,200.99,'z');
    let tp1=tp.0;
    let tp2=tp.1;
    let tp3=tp.2;

    //array
    let a=[1,2,3,4,5];
    let a_1=a[0];
    let a_2=a[1];





}
