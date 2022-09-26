
mac安装
一、安装rust编译器

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh\n

rustc -V

cargo -V

rustup default 1.58.1
```


二、执行rs文件
```shell
cargo new hello_rust

rustc main.rs

./main
```

三、注释
01 helloworld
02  数据类型
03  int
04  float
05  bool char
06  变量
07  常量
08  字符串
09  条件判断
10  循环
11  函数
12  元组
13  数组
14  所有权和转移
15  变量借用
16  切片
17  构造体
18  枚举
19  集合
20  接口 trait impl
21  io
22  文件操作
23  迭代器
24  闭包
25  线程
26  错误处理
27 指针
28 cargo 包管理
29 modules
30 scope变量绑定
31 别名 类型转换
32 match 匹配
33 另一种匹配方式 if let  while let
34 闭包的使用
35 异步等待



三、创表语句
```sql
create table employees
(
    first_name varchar,
    last_name  varchar,
    department varchar,
    salary     integer,
    age        integer,
    id         serial
);
```



### 四、问题记录

1、ld: library not found for -lpq 
 原因：本地没安装postgres
 brew install postgres


### 五、交叉编译

 .cargo/config

```txt
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
ar = "x86_64-w64-mingw32-gcc-ar"
```

linux
```shell
rustup target add x86_64-unknown-linux-musl
#下载arm库
rustup target add aarch64-unknown-linux-gnu

#安装musl-cross
brew install filosottile/musl-cross/musl-cross
```

脚本：
```shell
if [[ $(echo $0 | awk '/^//') == $0 ]]; then
    ABSPATH=$(dirname $0)
else
    ABSPATH=$PWD/$(dirname $0)
fi
cd ${ABSPATH}
cargo build --release --target x86_64-unknown-linux-musl
```


windows
    
脚本：
```shell
rustup target add x86_64-pc-windows-gnu
#安装mingw-w64
brew install mingw-w64
```

```shell
if [[ $(echo $0 | awk '/^//') == $0 ]]; then
    ABSPATH=$(dirname $0)
else
    ABSPATH=$PWD/$(dirname $0)
fi
cd ${ABSPATH}
cargo build --release --target x86_64-pc-windows-gnu
```