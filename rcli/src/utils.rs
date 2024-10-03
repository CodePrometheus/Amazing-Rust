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
use blake3::Hash;
use std::{fs, io::Read, path::Path};

pub fn verify_input(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

// dyn Read 表示一个实现了Read trait的动态分发对象，在运行时决定具体使用哪个类型，而不是在编译器确定
pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    if input == "-" {
        Ok(Box::new(std::io::stdin()))
    } else {
        Ok(Box::new(fs::File::open(input)?))
    }
}

pub fn get_content(input: &str) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn blake3hash(reader: &mut dyn Read, key: &[u8; 32]) -> Hash {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).unwrap();
    let ret = blake3::keyed_hash(key, &buf);
    ret
}

pub fn encode_hex(input: &[u8]) -> String {
    use std::fmt::Write;
    input
        .iter()
        .fold(String::with_capacity(input.len()), |mut output, b| {
            let _ = write!(output, "{b:02x}");
            output
        })
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(Into::into))
        .collect::<Result<Vec<u8>, Error>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input() {
        assert_eq!(verify_input("-"), Ok("-".into()));
        assert_eq!(verify_input("*"), Err("File does not exist"));
        assert_eq!(verify_input("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input("nonexistent"), Err("File does not exist"));
    }
}