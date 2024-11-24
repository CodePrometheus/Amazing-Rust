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

use std::time::Duration;

use axum::{extract::Request, routing::get, Router};
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    runtime,
    trace::{self, RandomIdGenerator, Tracer},
    Resource,
};
use tokio::{
    join,
    net::TcpListener,
    time::{sleep, Instant},
};
use tracing::{info, instrument, level_filters::LevelFilter, warn};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let console = fmt::Layer::new()
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .with_filter(LevelFilter::INFO);

    let file_appender = tracing_appender::rolling::daily("./logs", "ecosystem.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let log_file = fmt::Layer::new()
        .with_writer(non_blocking)
        .pretty()
        .with_filter(LevelFilter::INFO);

    let tracer = init_tracer()?;
    let otel = tracing_opentelemetry::layer().with_tracer(tracer);
    tracing_subscriber::registry()
        .with(console)
        .with(log_file)
        .with(otel)
        .init();

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    let app = Router::new().route("/", get(index_handler));
    info!("Staring server on {}", addr);
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

#[instrument(fields(http.uri = req.uri().path(), http.method = req.method().as_str()))]
async fn index_handler(req: Request) -> &'static str {
    let start = Instant::now();
    let sl = sleep(Duration::from_millis(11));
    let t1 = task1();
    let t2 = task1();
    let t3 = task1();
    join!(sl, t1, t2, t3);
    let elapsed = start.elapsed().as_millis() as u64;
    warn!(app.task_duration = elapsed, "task takes too long");
    "Hello, world~"
}

#[instrument]
async fn task1() {
    sleep(Duration::from_millis(10)).await;
}

#[instrument]
async fn task2() {
    sleep(Duration::from_millis(50)).await;
}

#[instrument]
async fn task3() {
    sleep(Duration::from_millis(30)).await;
}

fn init_tracer() -> anyhow::Result<Tracer> {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://192.168.1.8:4317"),
        )
        .with_trace_config(
            trace::config()
                .with_id_generator(RandomIdGenerator::default())
                .with_max_events_per_span(32)
                .with_max_attributes_per_span(64)
                .with_resource(Resource::new(vec![KeyValue::new(
                    "service.name",
                    "axum-tracing",
                )])),
        )
        .install_batch(runtime::Tokio)?;
    Ok(tracer)
}
