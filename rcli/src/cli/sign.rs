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

use crate::{get_content, get_reader, output_contents, process_decrypt, process_encrypt, process_generate, process_sign, process_verify, verify_input, CmdExecutor};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use clap::Parser;
use enum_dispatch::enum_dispatch;

impl CmdExecutor for SignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let signed = process_sign(&mut reader, &key, self.format)?;
        // base64 output
        let encoded = URL_SAFE_NO_PAD.encode(signed);
        println!("{}", encoded);
        Ok(())
    }
}

impl CmdExecutor for VerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let sig = get_content(&self.sig)?;
        let verified = process_verify(&mut reader, &key, &sig, self.format)?;
        println!("{}", verified);
        Ok(())
    }
}

impl CmdExecutor for GenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = process_generate(self.format);
        output_contents(&self.output, &key);
        Ok(())
    }
}

impl CmdExecutor for EncryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = String::from_utf8(get_content(&self.key)?)?;
        let encrypted = process_encrypt(&mut reader, &key)?;
        output_contents(&self.output, &encrypted);
        Ok(())
    }
}

impl CmdExecutor for DecryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = String::from_utf8(get_content(&self.key)?)?;
        let decrypted = process_decrypt(&mut reader, &key)?;
        output_contents(&self.output, &decrypted);
        Ok(())
    }
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum SignSubCommand {
    #[command(name = "sign", about = "Sign a message")]
    Sign(SignOpts),
    #[command(name = "verify", about = "Verify a signed message")]
    Verify(VerifyOpts),
    #[command(name = "generate", about = "Generate a key")]
    Generate(GenerateOpts),
    #[command(name = "encrypt", about = "Encrypt a message")]
    Encrypt(EncryptOpts),
    #[command(name = "decrypt", about = "Decrypt a message")]
    Decrypt(DecryptOpts),
}

#[derive(Debug, Parser)]
pub struct SignOpts {
    #[arg(short, long, value_parser = verify_input, default_value = "-")]
    pub input: String,
    #[arg(short, long, help = "key to sign with")]
    pub key: String,
    #[arg(short, long, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct VerifyOpts {
    #[arg(short, long, value_parser = verify_input, default_value = "-")]
    pub input: String,
    #[arg(short, long, help = "key to verify with")]
    pub key: String,
    #[arg(short, long, help = "signature to verify")]
    pub sig: String,
    #[arg(short, long, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct GenerateOpts {
    #[arg(short, long, help = "output file", default_value = "-")]
    pub output: String,
    #[arg(short, long, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct EncryptOpts {
    #[arg(short, long, value_parser = verify_input, default_value = "-")]
    pub input: String,
    #[arg(short, long, help = "key to encrypt with")]
    pub key: String,
    #[arg(short, long, help = "output file", default_value = "-")]
    pub output: String,
}

#[derive(Debug, Parser)]
pub struct DecryptOpts {
    #[arg(short, long, value_parser = verify_input, default_value = "-")]
    pub input: String,
    #[arg(short, long, help = "key to decrypt with")]
    pub key: String,
    #[arg(short, long, help = "output file", default_value = "-")]
    pub output: String,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

impl std::str::FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid text sign format")),
        }
    }
}