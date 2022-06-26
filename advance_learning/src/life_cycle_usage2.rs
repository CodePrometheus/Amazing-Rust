#[derive(Debug)]
struct Exc<'a> {
    part: &'a str, // 需要在每个引用上添加生命周期标注
}

pub fn main() {
    let nov = String::from("Call Nova. And so on");
    let first_sentence = nov.split('.')
        .next()
        .expect("Could not found a '.'");

    let i = Exc {
        part: first_sentence
    };
    println!("i: {:?}", i);
}