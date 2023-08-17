#![allow(dead_code)]
#![allow(unused_variables)]

use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;

fn main() {
    /*
        Vectors
    */
    let a = [1, 2, 3];
    let mut  v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let v2 = vec![1, 2, 3];

    match v.get(4) {
        Some(fifth) => println!("The fifth element is {}", fifth),
        None => println!("There is no fifth element")
    }

    for i in &v {
        println!("{}", i)
    }
    /*
        Strings
    */
    let hello = String::from("नमस्ते");
    for b in hello.bytes() {
        println!("{}", b)
    }
    println!("----------");
    for c in hello.chars() {
        println!("{}", c)
    }
    println!("----------");
    for g in hello.graphemes(true) {
        println!("{}", g)
    }
    println!("----------");

    /*
        HashMap
    */
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();
    scores.insert(blue, 10); // Owenership is passed to the hashmap
    scores.insert(yellow, 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    for (k, v) in &map {
        println!("{}: {}", k, v)
    }
}
