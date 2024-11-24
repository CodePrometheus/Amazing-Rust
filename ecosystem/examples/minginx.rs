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
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::{io, net::TcpStream};
use tracing::{info, level_filters::LevelFilter, warn};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Config {
    listener_addr: String,
    upstream_addr: String,
}

fn resolve_config() -> Config {
    Config {
        listener_addr: "0.0.0.0:8080".to_string(),
        upstream_addr: "0.0.0.0:8081".to_string(),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().pretty().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();
    let config = resolve_config();
    let config = Arc::new(config);
    info!("Upstream is {}", config.upstream_addr);
    info!("Listening on {}", config.listener_addr);

    let listener = tokio::net::TcpListener::bind(&config.listener_addr).await?;
    loop {
        let (client, addr) = listener.accept().await?;
        info!("Accepted connection from {}", addr);
        let cloned_config = config.clone();
        tokio::spawn(async move {
            let upstream = TcpStream::connect(&cloned_config.upstream_addr).await?;
            proxy(client, upstream).await?;
            Ok::<(), Error>(())
        });
    }
    #[allow(unreachable_code)]
    Ok::<(), Error>(())
}

async fn proxy(mut client: TcpStream, mut upstream: TcpStream) -> Result<()> {
    let (mut client_read, mut client_write) = client.split();
    let (mut upstream_read, mut upstream_write) = upstream.split();
    let client_to_upstream = io::copy(&mut client_read, &mut upstream_write);
    let upstream_to_client = io::copy(&mut upstream_read, &mut client_write);
    match tokio::try_join!(client_to_upstream, upstream_to_client) {
        Ok((n, m)) => info!(
            "proxied {} bytes from client to upstream, {} bytes from upstream to client",
            n, m
        ),
        Err(e) => warn!("error proxying: {:?}", e),
    }
    Ok(())
}
