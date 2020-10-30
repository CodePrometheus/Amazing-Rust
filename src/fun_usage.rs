pub fn main() {
    another_function(6, 8);

    let a = 5;

    let b = {
        let a = 3;
        // 表达式的结尾没有分号。如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值
        a + 1
    };

    println!("The value of b is: {}", b);
    println!("The value of a is: {}", a);

    let c = back();
    println!("The value of c is {}", c);

    let e = back_arg(9);
    println!("The value of e is {}", e);
}

// 在函数签名中，必须 声明每个参数的类型
fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// 函数可以向调用它的代码返回值。我们并不对返回值命名，但要在箭头（->）后声明它的类型
fn back() -> i32 {
    666
}

fn back_arg(d: i32) -> i32 {
    d + 1
}