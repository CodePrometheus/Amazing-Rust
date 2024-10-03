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

use std::path::PathBuf;
use clap::Parser;
use crate::verify_path;

#[derive(Debug, Parser)]
pub enum HttpSubcommand {
    #[command(name = "server", about = "Start an HTTP static file server")]
    Server(ServerOpts),
}

#[derive(Debug, Parser)]
pub struct ServerOpts {
    #[arg(short, long, help = "directory to serve", value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(long, help = "host to listen on", default_value = "0.0.0.0")]
    pub host: String,
    #[arg(short, long, help = "port to listen on", default_value = "8080")]
    pub port: u16,
}
