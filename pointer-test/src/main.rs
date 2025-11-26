use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use std::sync::Mutex;
use std::sync::RwLock;

use crate::List::{Cons, Nil};

// 定义一个链表枚举，用于演示循环引用
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

// 定义一个结构体，用于存储数据
#[derive(Debug)]
struct Data {
    value: i32,
}

fn main() {
    let b = Box::new(5);
    println!("b = {b}");

    let data = Rc::new(5);
    let data_clone = Rc::clone(&data);

    let data = Arc::new(5);
    let data_clone = Arc::clone(&data);

    let data = RefCell::new(5);
    let mut borrowed_data = data.borrow_mut();
    *borrowed_data = 10;

    let m = Mutex::new(5);
    let mut data = m.lock().unwrap();

    let lock = RwLock::new(5);
    let read_guard = lock.read().unwrap();


    // 创建一个 Rc 智能指针，共享数据
    let data = Rc::new(Data { value: 5 });

    // 克隆 Rc 智能指针，增加数据的引用计数
    let data_clone1 = Rc::clone(&data);
    let data_clone2 = Rc::clone(&data);

    // 输出数据的值和引用计数
    println!("Data value: {}", data.value);
    println!("Reference count: {}", Rc::strong_count(&data));

    // 打印克隆后的 Rc 智能指针
    println!("Data clone 1: {:?}", data_clone1);
    println!("Data clone 2: {:?}", data_clone2);

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
}
