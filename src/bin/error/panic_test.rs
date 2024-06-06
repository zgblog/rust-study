fn main() {
    let v = vec![1, 2, 3];
    v[99];
    // 手动调用宏，不可恢复错误
    panic!("crash and burn");
}