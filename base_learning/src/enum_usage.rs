enum Message {
    Quit,
    Move { x: i32, y: u32 },
    Write(String),
}

impl Message {
    fn call(&self) {
        println!("call")
    }
}

pub fn main() {
    let q = Message::Quit;
    let m = Message::Move { x: -32, y: 32 };
    m.call()
}