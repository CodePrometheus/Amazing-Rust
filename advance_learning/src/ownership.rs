pub fn main() {
    // 所有权存在的原因
    // 1.跟踪代码的哪些部分正在使用 heap 的哪些数据
    // 2.最小化 heap 上的重复数据
    // 3.清理 heap 上未使用的数据以避免空间不足

    // scope 变量有效范围 在一个 scope 中,变量的生命周期只在这个 scope 中
    // 一个值赋给其他变量时就会发生移动
    // 当一个包含 head 数据的变量离开作用域时,它的值就会被 drop 函数清理,除非数据的所有权移动到了另一个变量
    // 不能在同一作用域内同时拥有可变和不可变引用

    let mut s1 = String::from("hello");
    let len = calc(&mut s1);

    println!("The length of '{}' is {}.", s1, len);
}

// 引用某些值而不取得其所有权
fn calc(s: &mut String) -> usize {
    // s.push_str(", world"); `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    s.push_str(", world");
    s.len()
}