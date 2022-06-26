pub fn main() {
    let u1 = User {
        username: String::from("u1"),
        email: String::from("163@163.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("{:?}", u1);

    let a1 = (30, 50);
    println!("ret1: {}", area(a1));

    let a2 = Area {
        width: 30,
        len: 50,
    };
    println!("ret2: {}", calc_area(&a2));

    println!("Area: {:#?}", a2);
    println!("Area area_in_impl: {}", a2.area_in_impl());

    let a3 = Area {
        width: 20,
        len: 10,
    };
    println!("Area can_hold: {}", a2.can_hold(&a3))
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Area {
    width: u32,
    len: u32,
}

fn calc_area(area: &Area) -> u32 {
    area.width * area.len
}

fn area(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}

impl Area {
    fn area_in_impl(&self) -> u32 {
        self.width * self.len
    }

    fn can_hold(&self, other: &Area) -> bool {
        self.width > other.width &&
            self.len > other.len
    }
}