fn main() {
    println!("Hello, world!");
    let x=30;
    let img=image{
        width: dbg!(3*x),
        heigh: 30
    };
    let are=area(&img.width,&img.heigh);

    println!("{:?}",img);
    println!("{:#?}",img);
    dbg!(img);



}

#[derive(Debug)]
struct image {
    width: i32,
    heigh: i32,
}

fn area(width: &i32, heigh: &i32) ->i32{
    width*heigh
}
