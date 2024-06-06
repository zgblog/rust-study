fn main() {
    let mut v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];  // 从初始值构建
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    //
    let third: &i32 = &v[2];    // 访问：索引
    // 索引访问越界会导致恐慌，程序崩溃，使用get越界不会崩溃

    println!("The third element is {}", third);

    match v.get(2) {    // 访问：get
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element")
    }
    // 遍历
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    enum_vector_test();
}

fn enum_vector_test() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }
    let v = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(0.5f64),
        SpreadsheetCell::Text(String::from("Hello"))
    ];
    for vv in &v {
        match vv {
            SpreadsheetCell::Int(v) => println!("{}", v),
            SpreadsheetCell::Float(v) => println!("{}", v),
            SpreadsheetCell::Text(v) => println!("{}", v)
        }
    }
}