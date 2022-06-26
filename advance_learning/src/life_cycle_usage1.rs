pub fn main() {
    // {
    //     let r = 1;
    //     {
    //         let x = 5;
    //         r = &x; // 报错: r 的生命周期 > x 的生命周期
    //     }
    //     println!("r: {}", r)
    // }
    // solution:
    // let x = 5;
    // let r = &x;
    // println!("r: {}", r)

    let s1 = String::from("asdf");
    let s2 = "qwer";
    let ret = longest(s1.as_str(), s2);
    println!("ret: {}", ret)
}

// 生命周期的标注': 描述多个引用的生命周期间的关系, 但不影响生命周期
// 关联函数的不同参数以及返回值之间的生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}