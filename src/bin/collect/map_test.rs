use std::collections::HashMap;

fn main() {
    hash_new();
    hash_owner();
    hash_access();
    hash_update();
}

fn hash_new() {
    // 1. 使用HashMap.new创建
    let mut scores = HashMap::new();
    scores.insert("Blue", 22);
    scores.insert("Yellow", 33);

    // 2. 使用收集构建
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let init_scores = vec![22, 33];
    let scores: HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();

}

fn hash_owner() {
    let filed_name = String::from("Yellow");
    let filed_value = String::from("WWW");

    let mut map = HashMap::new();
    map.insert(filed_name, filed_value);    // 这里filed_name和field_value不再拥有所有权，而是转移到map
}

fn hash_access() {
    let mut scores = HashMap::new();
    scores.insert(String::from( "Blue"), 22);
    scores.insert(String::from("Yellow"), 33);

    let team_name = String::from("Blue");
    // 1. get访问
    let team_score = scores.get(&team_name);
    println!("{}", team_score.unwrap());

    // 2. 遍历
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn hash_update() {
    // 1. 覆盖
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 22);
    scores.insert(String::from("Blue"), 33);
    // 2. 只有在键没有时插入
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(15);
    scores.entry(String::from("Blue")).or_insert(20);

    println!("{:?}", scores);

    // 3. 更新值
    let mut map = HashMap::new();
    let text = "hello world wonderful world";
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}