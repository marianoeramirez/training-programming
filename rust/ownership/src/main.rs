use std::thread::spawn;
use std::sync::{Arc, Mutex};


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let string2 = "short sjdsk";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {}", result);

    let mut vector = vec![0,1,2,2,3,0,4,2];
    remove_element(&mut vector, 2);
    println!("The vector is: {:?}", vector);   
}

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let initial_lenth = nums.len();
    nums.retain(|&x| x != val);
    (initial_lenth - nums.len()) as i32
}