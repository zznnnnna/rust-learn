
mac安装
一、安装rust编译器

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh\n

rustc -V

cargo -V
```


二、执行rs文件
```shell
cargo new hello_rust

rustc main.rs

./main
```
