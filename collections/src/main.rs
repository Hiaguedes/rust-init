use collections::collections::list::vectors as vectors_module;
use std::collections::HashMap;
fn main() {
    let mut vector: Vec<i32> = Vec::new();
    let v = vec![1,2 ,3];

    vector.push(10);
    vector.push(20);
    vector.push(30);
    vector.push(40);
    vector.push(50);
    vector.pop();
    vector.push(60);

    // vectors_module::read_vector(vector);
    let result =  vectors_module::read_vector_string(vector);

    vectors_module::read_vector(v);
    let v = vec![9,3,-1];
    println!("{}", result);

    println!("{}", vectors_module::read_vector_string(v));

    let mut hello = String::from("hello");

    hello.push_str("  World");

    println!("{}", hello);

    println!("{}", hello + "!");

    let s1 = "s1";
    let s2 = "s2";
    let s3 = "s3";

    println!("{}", format!("{}-{}-{}", s1, s2, s3));

    let hello_slice = "hello";
    println!("{}", &hello_slice[0..4]);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{}", scores.get("Blue").unwrap());
    println!("{}", scores.entry(String::from("Blue")).or_insert(100));
    println!("{}", scores.entry(String::from("Grey")).or_insert(100));
    println!("{:?}", scores);
}
