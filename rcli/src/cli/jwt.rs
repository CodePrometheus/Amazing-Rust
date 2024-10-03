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

use crate::CmdExecutor;
use clap::Parser;
use enum_dispatch::enum_dispatch;

impl CmdExecutor for JWTEncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let token = crate::process_jwt_sign(&self.key, &self.exp, self.aud, self.iss, self.sub)?;
        println!("Sign JWT: \n{}", token);
        Ok(())
    }
}

impl CmdExecutor for JWTDecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let verified = crate::process_jwt_verify(&self.key, &self.token, self.aud);
        println!("Verify JWT: {}", verified.is_ok());
        Ok(())
    }
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum JwtSubcommand {
    #[command(name = "sign", about = "Sign a JWT")]
    Sign(JWTEncodeOpts),
    #[command(name = "verify", about = "Verify a JWT")]
    Verify(JWTDecodeOpts),
}

#[derive(Debug, Parser)]
pub struct JWTEncodeOpts {
    #[arg(short, long, help = "key to sign with", default_value = "secret")]
    pub key: String,
    #[arg(short, long, help = "audience")]
    pub aud: Option<String>,
    #[arg(short, long, help = "subject", default_value = "1d")]
    pub exp: String,
    #[arg(short, long, help = "issuer")]
    pub iss: Option<String>,
    #[arg(short, long, help = "subject")]
    pub sub: Option<String>,
}

#[derive(Debug, Parser)]
pub struct JWTDecodeOpts {
    #[arg(short, long, help = "key to verify with", default_value = "secret")]
    pub key: String,
    #[arg(short, long, help = "token to verify")]
    pub token: String,
    #[arg(short, long, help = "audience")]
    pub aud: Option<String>,
}