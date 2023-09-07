/*
3210103818
杨朗骐
*/
extern crate std;

//Exercise 1 on page 41
use std::collections::HashMap;

macro_rules! hash_map {
    ($($key:expr => $value:expr),*) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}
fn test_exercise_1() {
    let map = hash_map!(
        "one" => 1,
        "two" => 2,
        "three" => 3
    );
    println!("Exercise 1.\nThe hash map is {:?}", map);
}




//Exercise 2 on page 53
/*
功能：
1.可用“*”解引用
2.在多个地方共享只读数据。当最后一个引用离开作用域时，Rc<T>会自动释放分配的内存
3.可通过strong_count()查看引用计数
*/
use std::alloc;
use std::ops::Deref;
struct MyRc<T> {
    value: T,
    ref_count: *mut usize,
}

impl<T> MyRc<T> {
    fn new(value: T) -> Self {
        unsafe {
            let cnt_layout = alloc::Layout::new::<usize>();
            let cnt_ptr = alloc::alloc(cnt_layout);
            *(cnt_ptr as *mut usize) = 1usize;
            MyRc {
                value,
                ref_count: cnt_ptr as *mut usize,
            }
        }
    }
    fn clone(&self) -> Self
    where
        T: Clone,
    {
        unsafe {
            *self.ref_count += 1;
        }
        MyRc {
            value: self.value.clone(),
            ref_count: self.ref_count,
        }
    }
    fn strong_count(&self) -> usize {
        unsafe { *(self.ref_count) }
    }
}
impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            if (*self.ref_count) <= 1 {
                let cnt_layout = alloc::Layout::new::<usize>();
                alloc::dealloc(self.ref_count as *mut u8, cnt_layout);
                println!("MyRc Dropped")
            } else {
                (*(self.ref_count)) -= 1;
            }
        }
    }
}
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




//Exercise 3 on page 54

use std::cell::RefCell;
#[derive(Debug)]
struct Stack<T> {
    stack: RefCell<Vec<T>>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            stack: RefCell::new(Vec::new()),
        }
    }
    fn push(&self, value: T) {
        self.stack.borrow_mut().push(value);
    }
    fn pop(&self) -> Option<T> {
        self.stack.borrow_mut().pop()
    }
}
fn test_exercise_3() {
    println!("\nExercise 3.");
    let stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("Pop.Poped value:{:?}", stack.pop());
    println!("Pop.Poped value:{:?}", stack.pop());
    stack.push(4);
    println!("Pop.Poped value:{:?}", stack.pop());
    println!("Pop.Poped value:{:?}", stack.pop());
    println!("Pop.Poped value:{:?}", stack.pop());
}

fn main() {
    println!("Test:");
    //Exercise 1 on page 41
    test_exercise_1();

    //Exercise 2 on page 53
    test_exercise_2();

    //Exercise 3 on page 54
    test_exercise_3();
}
