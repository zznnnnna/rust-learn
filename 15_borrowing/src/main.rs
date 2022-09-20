fn main() {
    println!("Hello, world!");

    let vec = vec!["name", "zzn", "rex"];
    borrow_vec(&vec);
    println!("borrow after is {:?}", vec);


    let mut vec2 = vec!["a", "b"];
    borrow_vec_mut(&mut vec2);
    println!("vec2 is {:?}",vec2);

}

fn borrow_vec(vec: &Vec<&str>) {
    println!("{:?}", vec);
}

fn borrow_vec_mut(vec: &mut Vec<&str>) {
    println!("{:?}", vec);
    for v in vec.iter_mut() {
       *v= match v {
            &mut "a"=>"aaaa",
            _=>*v
        }
    }
}