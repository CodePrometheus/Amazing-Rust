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

// rcli csv -i input.csv -o output.json --header -d ','

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use rcli::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, &output, opts.format, opts.delimiter, opts.header)?;
        }
        SubCommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
        }
        SubCommand::Base64(subcommand) => match subcommand {
            Base64SubCommand::Encode(opts) => {
                let mut reader = get_reader(&opts.input)?;
                process_base64(&mut reader, &opts.format, Base64Action::Encode)?;
            }
            Base64SubCommand::Decode(opts) => {
                let mut reader = get_reader(&opts.input)?;
                process_base64(&mut reader, &opts.format, Base64Action::Decode)?;
            }
        },
        SubCommand::Sign(subcommand) => match subcommand {
            SignSubCommand::Sign(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let signed = process_sign(&mut reader, &key, opts.format)?;
                // base64 output
                let encoded = URL_SAFE_NO_PAD.encode(signed);
                println!("{}", encoded);
            }
            SignSubCommand::Verify(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let sig = get_content(&opts.sig)?;
                let verified = process_verify(&mut reader, &key, &sig, opts.format)?;
                println!("{}", verified);
            }
            SignSubCommand::Generate(opts) => {
                let key = process_generate(opts.format);
                output_contents(&opts.output, &key);
            }
            SignSubCommand::Encrypt(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = String::from_utf8(get_content(&opts.key)?)?;
                let encrypted = process_encrypt(&mut reader, &key)?;
                output_contents(&opts.output, &encrypted);
            }
            SignSubCommand::Decrypt(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = String::from_utf8(get_content(&opts.key)?)?;
                let decrypted = process_decrypt(&mut reader, &key)?;
                output_contents(&opts.output, &decrypted);
            },
        },
        SubCommand::Http(subcommand) => match subcommand {
            HttpSubcommand::Server(opts) => {
                process_http_server(opts.dir, &opts.host, opts.port).await?;
            }
        },
    }
    Ok(())
}
