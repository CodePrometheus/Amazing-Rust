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

use crate::{blake3hash, decode_hex, encode_hex, process_genpass, TextSignFormat};
use anyhow::Result;
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use std::collections::HashMap;
use std::io::Read;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit},
    ChaCha20Poly1305, Nonce,
};

pub fn process_sign(
    reader: &mut dyn Read,
    key: &[u8],
    format: TextSignFormat,
) -> Result<Vec<u8>> {
    let signer: Box<dyn TextSigner> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519::try_new(key)?),
    };

    signer.sign(reader)
}

pub fn process_verify(
    reader: &mut dyn Read,
    key: &[u8],
    sig: &[u8],
    format: TextSignFormat,
) -> Result<bool> {
    let verifier: Box<dyn TextVerifier> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519::try_new(key)?),
    };
    verifier.verify(reader, sig)
}

pub fn process_generate(format: TextSignFormat) -> String {
    match format {
        TextSignFormat::Blake3 => Blake3::generate().unwrap(),
        TextSignFormat::Ed25519 => Ed25519::generate().unwrap(),
    }
}

pub fn process_encrypt(input: &mut dyn Read, key: &str) -> Result<String> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let cipher = ChaCha20Poly1305::new_from_slice(decode_hex(key)?.as_slice())?;
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, buf.as_bytes())
        .map_err(|_| anyhow::anyhow!("encrypt error"))?;

    let mut output = Vec::new();
    output.extend_from_slice(nonce.as_slice());
    output.extend_from_slice(ciphertext.as_slice());

    Ok(encode_hex(output.as_slice()))
}

pub fn process_decrypt(input: &mut dyn Read, key: &str) -> Result<String> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let buf = decode_hex(buf.trim())?;

    let cipher = ChaCha20Poly1305::new_from_slice(decode_hex(key)?.as_slice())?;
    let nonce = Nonce::from_slice(&buf[0..12]);

    let plaintext = cipher
        .decrypt(nonce, &buf[12..])
        .map_err(|_| anyhow::anyhow!("decrypt error"))?;

    Ok(String::from_utf8(plaintext)?)
}

pub trait TextSigner {
    // signer could sign any input data
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerifier {
    // verifier could verify any input data
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool>;
}

#[derive(Clone)]
pub struct Blake3 {
    key: [u8; 32],
}

#[derive(Clone)]
pub struct Ed25519 {
    key: SigningKey,
}

impl Blake3 {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        println!("Blake3|try_new = {:?}", key);
        Ok(Self::new(key))
    }

    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }
    
    fn generate() -> Result<String> {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        Ok(encode_hex(&signing_key.to_bytes()))
    }

    #[allow(dead_code)]
    fn generate_old() -> Result<HashMap<&'static str, Vec<u8>>> {
        let key = process_genpass(32, true, true, true, true)?;
        let mut map = HashMap::new();
        map.insert("blake3.txt", key.as_bytes().to_vec());
        Ok(map)
    }
}

impl Ed25519 {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }

    fn new(key: [u8; 32]) -> Self {
        Self {
            key: SigningKey::from_bytes(&key),
        }
    }

    #[allow(dead_code)]
    fn generate_old() -> Result<HashMap<&'static str, Vec<u8>>> {
        let mut csprng = OsRng;
        let sk: SigningKey = SigningKey::generate(&mut csprng);
        let pk: VerifyingKey = (&sk).into();
        let mut map = HashMap::new();
        map.insert("ed25519.sk", sk.to_bytes().to_vec());
        map.insert("ed25519.pk", pk.to_bytes().to_vec());

        Ok(map)
    }

    fn generate() -> Result<String> {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        Ok(encode_hex(&signing_key.to_keypair_bytes()))
    }
}

impl TextSigner for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        Ok(blake3hash(reader, &self.key).as_bytes().to_vec())
    }
}

impl TextVerifier for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        Ok(blake3hash(reader, &self.key).as_bytes() == sig)
    }
}

impl TextSigner for Ed25519 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature = self.key.sign(&buf);
        Ok(signature.to_bytes().to_vec())
    }
}

impl TextVerifier for Ed25519 {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = (&sig[..64]).try_into()?;
        let signature = Signature::from_bytes(sig);
        Ok(self.key.verify(&buf, &signature).is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

    #[test]
    fn test_blake3_generate() {
        let key = Blake3::generate();
        println!("key = {:?}", key);
        assert!(key.is_ok());
    }

    #[test]
    fn test_blake3_try_new() {
        let key = Blake3::generate().unwrap();
        let blake3 = Blake3::try_new(&key);
        assert!(blake3.is_ok());
    }

    #[test]
    fn test_ed25519_generate() {
        let key = Ed25519::generate();
        assert!(key.is_ok());
    }

    #[test]
    fn test_ed25519_try_new() {
        let key = Ed25519::generate().unwrap();
        let edd25519 = Ed25519::try_new(&key);
        assert!(edd25519.is_ok());
    }


    const KEY: &[u8] = include_bytes!("../../fixtures/blake3.txt");

    #[test]
    fn test_process_sign_blake3() -> Result<()> {
        let mut reader = "hello".as_bytes();
        let mut reader1 = "hello".as_bytes();
        let format = TextSignFormat::Blake3;
        let sig = process_sign(&mut reader, KEY, format)?;
        let ret = process_verify(&mut reader1, KEY, &sig, format)?;
        assert!(ret);
        Ok(())
    }

    #[test]
    fn test_process_verify_blake3() {
        let mut reader = "hello".as_bytes();
        let format = TextSignFormat::Blake3;
        let sig = "33Ypo4rveYpWmJKAiGnnse-wHQhMVujjmcVkV4Tl43k";
        let sig = URL_SAFE_NO_PAD.decode(sig).expect("decode failed");
        let ret = process_verify(&mut reader, KEY, &sig, format).expect("verify failed");
        assert!(ret)
    }
    
    #[test]
    fn test_encrypt() {
        let input = &mut "hello,world".as_bytes();
        let key = "d15b212054ab60da12d67534d79d06f432bc1d7be2b5902297189639078c4a38";
        let encrypted = process_encrypt(input, key).unwrap();
        let descrypted = process_decrypt(&mut encrypted.as_bytes(), key).unwrap();
        assert_eq!(descrypted, "hello,world");
    }
    
    #[test]
    fn test_decrypt() {
        let input = &mut "e347ffe968aa3ed7601b013c2c1db18f0b77eb4293b388f3b667929368297a806c57408e783fd6".as_bytes();
        let key = "d15b212054ab60da12d67534d79d06f432bc1d7be2b5902297189639078c4a38";
        let resp = process_decrypt(input, key).unwrap();
        assert_eq!(resp, "hello,world");
    }
}