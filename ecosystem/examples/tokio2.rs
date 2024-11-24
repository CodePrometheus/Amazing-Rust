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

use anyhow::{Error, Result};
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel(32);
    let handler = worker(rx);

    tokio::spawn(async move {
        let mut i = 0;
        loop {
            i += 1;
            println!("Send task: {i}!");
            tx.send(format!("task {i}")).await?;
        }
        #[allow(unreachable_code)]
        Ok::<(), Error>(())
    });
    handler.join().unwrap();

    Ok(())
}

fn worker(mut rx: mpsc::Receiver<String>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        while let Some(s) = rx.blocking_recv() {
            let ret = handle_future(s);
            println!("ret: {}", ret)
        }
    })
}

fn handle_future(s: String) -> String {
    thread::sleep(Duration::from_millis(800));
    blake3::hash(s.as_bytes()).to_string()
}
