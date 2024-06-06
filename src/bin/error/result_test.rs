use std::{fs::File, io::{self, ErrorKind, Read}};

fn main() {
    // 返回的Result
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // 没有文件时尝试创建文件
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error)
        }
    };
}

fn result_2() {
    // 消除match
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn result_3() {
    // 解包，unwrap，如果Result值为OK，会返回OK中的值，否则会panic
    let f = File::open("hello.txt").unwrap();
}

fn result_4() {
    // expect提供一个自定义的错误信息
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

/**
 * 一个函数使用 match 将错误返回给代码调用者
 */
fn result_5() -> Result<String, io::Error> {
    let f = File::open("Hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e) // 提早返回
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

/**
 * 一个使用 ? 运算符向调用者返回错误的函数
 */
fn result_6() -> Result<String, io::Error> {
    let mut s = String::new();
    // 链式，当出现错误时，会自动返回Err
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}