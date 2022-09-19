use std::fs::File;
use std::io::Read;

fn main() {

    let file=File::open("/Users/n/rust-learn/file_22/src/app.txt").unwrap();

    print!("文件打开成功 file {:?}",file);

}
