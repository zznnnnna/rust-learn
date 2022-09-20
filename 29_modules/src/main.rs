use mylib::lib_person::getlibage;

// pub mod person {
//     pub fn getName() {
//         println!("my name is  ame");
//     }
//
//     fn getAge() {
//         println!("my age is  ge");
//     }
// }
/*
  mod module_name{
  pub fn function_name{}

  }

  use 公开模块::函数名称
   */
fn main() {
    //use person::getName as d;
    // d();
    // 1.在当前目录执行 cargo new --lib mylib
    //2.修改Cargo.toml  mylib = { path = "../29_modules/mylib/" }

    getlibage(24);
}
