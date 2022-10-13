fn main() {
    // println!("Hello, world!");

    let a=[1,2,3,4,5];
    for i in a{
        println!("i is {i}");
    }

    for num in (1..4).rev(){
        println!("num us {num}");
    }

    let mut w=2;
    while w<5  {
        println!("w is {w}");
        w=w+1;
    }

    let mut l=1;
    loop {
        println!("l is {l}");
        l=l+1;
        if l>10 { break;};
    }
}


