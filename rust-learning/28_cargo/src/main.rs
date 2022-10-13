fn main() {
    //适用cargo 进行包管理
    println!("Hello, world!");
    /*
    # n @ bogon in ~/rust-learn on git:master x [15:26:09]
    $ cargo --version
    cargo 1.63.0 (fd9c4297c 2022-07-01)

    # n @ bogon in ~/rust-learn on git:master x [15:26:44]
    $ cargo --list
    Installed Commands:
        add                  Add dependencies to a Cargo.toml manifest file
        b                    alias: build
    */

    /*
    cargo new	在当前目录下新建一个 cargo 项目
    cargo check	分析当前项目并报告项目中的错误，但不会编译任何项目文件
    cargo build	编译当前项目
    cargo run	编译并运行文件 src/main.rs
    cargo clean	移除当前项目下的 target 目录及目录中的所有子目录和文件
    cargo update	更新当前项目中的 Cargo.lock 文件列出的所有依赖
     */
}
