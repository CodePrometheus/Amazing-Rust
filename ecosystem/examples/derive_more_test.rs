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
use derive_more::{Add, Display, From, Into};

#[derive(PartialEq, Clone, Copy, From, Add, Into, Display)]
struct MyInt(i32);

#[derive(PartialEq, From)]
struct Point2D {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, From, Add, Display)]
enum MyEnum {
    #[display(fmt = "int: {_0}")]
    Int(i32),
    Uint(u32),
    #[display(fmt = "nothing")]
    Nothing,
}

fn main() -> Result<()> {
    let my_int: MyInt = 10.into();
    let v = my_int + 20.into();
    let v1: i32 = v.into();

    println!("my_int: {}, v: {}, v1: {}", my_int, v, v1);

    let e: MyEnum = 10i32.into();
    let e1: MyEnum = 20u32.into();
    let e2 = MyEnum::Nothing;
    println!("e: {:?}, e1: {:?}, e2: {:?}", e, e1, e2);

    Ok(())
}
