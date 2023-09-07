# Assignment3-Rust
3210103818  
杨朗骐
# 测试
cargo run  
其它测试不再赘述，这里说明一下 Exercise 2: MyRc 的测试  
```rust
fn test_exercise_2() {
    println!("\nExercise 2.");
    let five = MyRc::new(5);//创建一个给定值的MyRc实例“five”
    //测试Deref Trait和strong_count()，输出此时“five”的内部数据及引用计数
    println!(
        "The original MyRc is {}.It is referenced {} times.",
        *five,
        five.strong_count()
    );
    {
        let five1 = five.clone();//测试克隆函数，获得“five1”
        //输出克隆后“five”和“five1”的内容和引用计数，此时引用计数应都为2
        println!(
            "The original MyRc is {}.It is referenced {} times.",
            *five,
            five.strong_count()
        );
        println!(
            "The new MyRc is {}.It is referenced {} times.",
            *five1,
            five1.strong_count()
        );
    }
    //测试drop，此时“five1”离开作用域应drop，引用计数应为1
    println!(
        "The original MyRc is {}.It is referenced {} times.",
        *five,
        five.strong_count()
    );
}//“five”应drop，引用计数为0，回收空间并打印"MyRc Dropped"
```
# 结果如下
<img width="556" alt="截屏2023-09-07 下午1 50 13" src="https://github.com/westoutlegenddog/Assignment3-Rust/assets/103580732/bd4c693c-135d-40ea-a516-8ce3552930be">

