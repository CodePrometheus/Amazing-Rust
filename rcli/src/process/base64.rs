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

use crate::cli::{Base64Action, Base64Format};
use crate::get_reader;
use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use std::io::Read;

pub fn process_encode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    println!("encoded = {}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();
    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };
    let decoded = String::from_utf8(decoded)?;
    println!("process_decode|decoded = {}", decoded);
    Ok(())
}

pub fn process_base64(
    input: &mut dyn Read,
    format: &Base64Format,
    action: Base64Action,
) -> Result<String> {
    let mut buf = String::new();
    input.read_to_string(&mut buf).expect("Failed to read input");
    let buf = buf.trim();

    let result = match (format, action) {
        (Base64Format::Standard, Base64Action::Encode) => STANDARD.encode(buf),
        (Base64Format::Standard, Base64Action::Decode) => {
            String::from_utf8(STANDARD.decode(buf)?)?
        }

        (Base64Format::UrlSafe, Base64Action::Encode) => URL_SAFE_NO_PAD.encode(buf),
        (Base64Format::UrlSafe, Base64Action::Decode) => {
            String::from_utf8(URL_SAFE_NO_PAD.decode(buf)?)?
        }
    };
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::Base64Action;

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/b64.txt";
        let format = Base64Format::UrlSafe;
        process_decode(input, format).unwrap();
    }

    #[test]
    fn test_process_base64_encode_standard() {
        let input = "hello world";
        let mut input = input.as_bytes();
        let result =
            process_base64(&mut input, &Base64Format::Standard, Base64Action::Encode).unwrap();
        assert_eq!(result, "aGVsbG8gd29ybGQ=");
    }

    #[test]
    fn test_process_base64_decode_standard() {
        let input = "aGVsbG8gd29ybGQ=";
        let mut input = input.as_bytes();
        let result =
            process_base64(&mut input, &Base64Format::Standard, Base64Action::Decode).unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_process_base64_encode_urlsafe() {
        let input = "hello world";
        let mut input = input.as_bytes();
        let result =
            process_base64(&mut input, &Base64Format::UrlSafe, Base64Action::Encode).unwrap();
        assert_eq!(result, "aGVsbG8gd29ybGQ");
    }

    #[test]
    fn test_process_base64_decode_urlsafe() {
        let input = "aGVsbG8gd29ybGQ";
        let mut input = input.as_bytes();
        let result =
            process_base64(&mut input, &Base64Format::UrlSafe, Base64Action::Decode).unwrap();
        assert_eq!(result, "hello world");
    }
}
