// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use anyhow::Result;
use serde::Serialize;
use strum::{
    Display, EnumCount, EnumDiscriminants, EnumIs, EnumIter, EnumString, IntoEnumIterator,
    IntoStaticStr, VariantNames,
};

#[allow(unused)]
#[derive(Display, Debug, Serialize)]
enum Color {
    #[strum(serialize = "redred")]
    Red,
    Green {
        range: usize,
    },
    Blue(usize),
    Yellow,
    #[strum(to_string = "purple with {sat} saturation")]
    Purple {
        sat: usize,
    },
}

#[derive(
    Debug, EnumString, EnumCount, EnumDiscriminants, EnumIter, EnumIs, IntoStaticStr, VariantNames,
)]
#[allow(unused)]
enum MyEnum {
    A,
    B(String),
    C,
    D,
}

fn main() -> Result<()> {
    println!("{:?}", MyEnum::VARIANTS);
    MyEnum::iter().for_each(|v| println!("{:?}", v));
    println!("total: {:?}", MyEnum::COUNT);

    let my_enum = MyEnum::B("hello".to_string());
    println!("{:?}", my_enum.is_b());
    let s: &'static str = my_enum.into();
    println!("{}", s);

    let red = Color::Red;
    let green = Color::Green { range: 10 };
    let blue = Color::Blue(20);
    let yellow = Color::Yellow;
    let purple = Color::Purple { sat: 30 };

    println!(
        "red: {}, green: {}, blue: {}, yellow: {}, purple: {}",
        red, green, blue, yellow, purple
    );

    let red_str = serde_json::to_string(&red)?;
    println!("{}", red_str);

    Ok(())
}
