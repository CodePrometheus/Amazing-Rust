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

use std::thread;
use std::time::Duration;
use tokio::fs;
use tokio::runtime::Builder;
use tokio::time::sleep;

fn main() {
    let handler = thread::spawn(|| {
        let rt = Builder::new_current_thread().enable_all().build().unwrap();
        rt.spawn(async {
            println!("Future 1");
            let content = fs::read_to_string("Cargo.toml").await.unwrap();
            println!("content len: {}", content.len())
        });
        rt.spawn(async {
            println!("Future 2");
            let ret = handle_future_2("Future 2".to_string());
            println!("ret: {}", ret)
        });
        rt.block_on(async {
            sleep(Duration::from_millis(900)).await;
            println!("Future 3"); 
        })
    });
    handler.join().unwrap();
}

fn handle_future_2(s: String) -> String {
    thread::sleep(Duration::from_millis(800));
    blake3::hash(s.as_bytes()).to_string()
}
