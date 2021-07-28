use std::collections::HashMap;
use std::convert::TryInto;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Red"), 20);

    let keys = vec![String::from("Blue"), String::from("Red")];
    let values = vec![10, 20];
    let scores: HashMap<_, _> = keys.iter().zip(values.iter()).collect();
    let key = String::from("Blue");

    if let Some(v) = scores.get(&key) {
        println!("{:?}", v);
    }

    let key = String::from("a");
    let v = scores.get(&key);
    match v {
        Some(v) => {
            println!("{:?}", v)
        }
        _ => println!("no")
    }
    // 遍历
    for (key, value) in &scores {
        println!("{},{}", key, value)
    }
    // 直接插入值
    let mut ss = HashMap::new();
    // ss.insert(String::from("one"),1);
    ss.insert(String::from("two"), 2);
    ss.insert(String::from("three"), 3);
    // ss.insert(String::from("one"),3);
    ss.entry(String::from("one")).or_insert(4);

    let text = "hello world wonderful world";
    let mut count_map = HashMap::new();
    for word in text.split_whitespace() {
        let count = count_map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", ss);
    println!("{:?}", count_map);
    let a = two_sum(vec![2, 7, 11, 15], 9);
    println!("{:#?}", a);
}


pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut sum_map: HashMap<i32, i32> = HashMap::new();
    for (idx, num) in nums.iter().enumerate() {
        if let Some(v) = sum_map.get(&(&target - *num)) {
            return  vec![idx.try_into().unwrap(), *v];
        } else {
            sum_map.entry(*num).or_insert(idx.try_into().unwrap());
        }
    }
    vec![]
}