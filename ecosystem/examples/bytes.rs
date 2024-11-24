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
use bytes::{BufMut, BytesMut};

fn main() -> Result<()> {
    let mut buf = BytesMut::with_capacity(1024);
    buf.extend_from_slice(b"hello,world|");
    buf.put(&b"goodbye"[..]);
    buf.put_i32(0x3f);
    println!("buf = {:?}", buf);
    let a = buf.split();
    println!("a = {:?}", a);
    let mut b = a.freeze();
    println!("b = {:?}", b);
    let c = b.split_to(12);
    println!("c = {:?}", c);
    println!("buf = {:?}", buf);
    Ok(())
}