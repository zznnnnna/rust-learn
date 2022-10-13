use std::fs::{File, OpenOptions, remove_file};
use std::io::{Read, Write};

fn main() {
    let file = File::open("/Users/n/rust-learn/file_22/src/app.txt").unwrap();

    println!("文件打开成功 file {:?}", file);

    let file2 = File::create("/Users/n/rust-learn/file_22/src/app2.txt").expect("创建不成功");

    println!("文件创建成功 {:?}", file2);

    remove_file("/Users/n/rust-learn/file_22/src/app2.txt").expect("文件删除失败");

    println!("文件删除成功");

    let mut file3=OpenOptions::new().append(true).open("/Users/n/rust-learn/file_22/src/app.txt").expect("失败");

    file3.write("\n 我要添加内容".as_bytes()).expect("失败");

    file3.write_all("hhhh \
    hhhhh".as_bytes()).expect("失败写入");

    print!("写入成功");

    let mut file4=File::open("/Users/n/rust-learn/file_22/src/app.txt").unwrap();
    let mut content=String::new();
    file4.read_to_string(&mut content).unwrap();
    print!("{} ",content);
}
