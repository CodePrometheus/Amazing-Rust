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

mod csv;
mod passwd;
mod base64;
mod sign;
mod http;
mod jwt;

pub use base64::*;
pub use csv::*;
pub use http::*;
pub use passwd::*;
pub use sign::*;
pub use jwt::*;

use clap::{command, Parser};

#[derive(Debug, Parser)]
#[command(version, name = "rcli", author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Base64 Encode/Decode")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Text Sign/Verify")]
    Sign(SignSubCommand),
    #[command(subcommand, about = "Http server")]
    Http(HttpSubcommand),
    #[command(subcommand, about = "JWT sign/verify")]
    JWT(JwtSubcommand),
}