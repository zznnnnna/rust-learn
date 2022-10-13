fn main() {
    let spend =1;
    {
        let spend ="2";
        println!("spend inner is {}",spend);

    }
    println!("spend outter is {}",spend);

    let x;
    {
        let i=99;
        x=i*2;
    }

    println!("x is {}",x);


    let mut spend2= 200;
    let spend3=&spend2;
    spend2= *Box::new(20);
    println!("{}",spend2);

}
