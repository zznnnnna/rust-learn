fn main() {
    println!("Hello, world!");
    let ipv4 = IpAddr::V4(String::from("ipv4"));
    println!("{:#?}",ipv4);

    let somes=Some(5);
    assert_eq!(somes.unwrap(),5);
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

