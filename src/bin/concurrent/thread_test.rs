use std::thread;
use std::time::Duration;

fn thread_test1() {
    // 创建一个新的线程并执行
   let handle =  thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    handle.join().unwrap();
}

fn thread_test2() {
    let v = vec![1, 2, 3];

    // move 所有权转移
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn main() {
    thread_test1();
    thread_test2();
}
