pub fn main() {
    // let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(1);
    println!("{:?}", v);

    for i in &mut v {
        *i += 50
    }
    for i in &v {
        println!("{}", i)
    }
}