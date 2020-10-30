pub fn main() {
    // 变量是默认不可变的，加mut使其可变
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6;
    let x = 6;
    println!("The value of x is: {}", x);

    // 不允许对常量使用 mut，常量不光默认不能变，它总是不能变，必须⚠注明值的类型
    // 命名规范是使用下划线分隔的大写字母单词，并且可以在数字字面值中插入下划线来提升可读性，如下100,000
    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);

    // 再次使用 let 时，实际上创建了一个新变量，我们可以改变值的类型，但复用这个名字
    let y = 7;
    let y = y + 1;
    let y = y / 2;
    println!("The value of y is: {}", y);
}