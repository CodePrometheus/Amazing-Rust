pub fn main() {
    let my_string = String::from("Hello");
    println!("{}", first_world(&my_string[..]));

    let my_string_literal = "world";
    println!("{}", first_world(my_string_literal));
}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

