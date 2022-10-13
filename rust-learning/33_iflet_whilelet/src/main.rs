fn main() {
    let s = Some("我是Rex");

    let s1: Option<i32> = None;
    let s2: Option<i32> = None;

    if let Some(i) = s {
        println!("你是谁？{:?}", i);
    }

    if let Some(i) = s1 {
        println!("Matched {:?}", i);
    } else {
        println!("不匹配")
    }

    // 提供另一种失败情况下的条件。
    let flag = false;
    if let Some(i) = s2 {
        println!("Matched {:?}!", i);
        // 解构失败。使用 `else if` 来判断是否满足上面提供的条件。
    } else if flag {
        println!("不匹配s2");
    } else {
        // 条件的值为 false。于是以下是默认的分支：
        println!("默认分支");
    };


    // 将 `optional` 设为 `Option<i32>` 类型
    let mut num = Some(0);

// 当 `let` 将 `optional` 解构成 `Some(i)` 时，就
// 执行语句块（`{}`）。否则就 `break`。
    while let Some(i) = num {
        if i > 9 {
            println!("{},quit!",i);
            num = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            num = Some(i + 1);
        }
    }
}
