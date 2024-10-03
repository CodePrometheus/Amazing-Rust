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

use anyhow::Result;
use axum::http::HeaderMap;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::net::Ipv6Addr;
use std::str::FromStr;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::{fs, net::TcpListener};
use tower_http::services::ServeDir;
use tracing::info;

#[derive(Debug)]
pub struct AppState {
    pub dir: PathBuf,
}

pub async fn process_http_server(dir: impl AsRef<std::path::Path>, host: &str, port: u16) -> Result<()> {
    let addr = if Ipv6Addr::from_str(&host).is_ok() {
        SocketAddr::from_str(&format!("[{}]:{}", host, port))?
    } else {
        SocketAddr::from_str(&format!("{}:{}", host, port))?
    };
    info!("Listening on {}", addr);
    let state = Arc::new(AppState {
        dir: dir.as_ref().to_path_buf(),
    });

    let serve_dir = ServeDir::new(dir.as_ref())
        .append_index_html_on_directories(true)
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_gzip()
        .precompressed_zstd();

    let app = Router::new()
        .route("/", get(file_index))
        .route("/*path", get(file_handler))
        .nest_service("/tower", serve_dir)
        .with_state(state);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn file_index(State(state): State<Arc<AppState>>) -> (StatusCode, HeaderMap, String) {
    file_handler(State(state.clone()), Path("/".to_string())).await
}

async fn file_handler(
    State(state): State<Arc<AppState>>,
    Path(path_arg): Path<String>,
) -> (StatusCode, HeaderMap, String) {
    let path = std::path::Path::new(&state.dir).join(path_arg.trim_start_matches('/'));
    let mut headers = HeaderMap::new();

    if path.is_dir() {
        match path.read_dir() {
            Ok(entries) => {
                let entries = entries
                    .filter_map(|entry| {
                        entry.ok().map(|entry| {
                            let path = entry.path();
                            let name = path.file_name().unwrap().to_string_lossy().to_string();
                            let is_dir = path.is_dir();
                            (name, is_dir)
                        })
                    })
                    .collect::<Vec<_>>();

                let html = entries
                    .iter()
                    .map(|(name, is_dir)| {
                        let src = std::path::Path::new(&path_arg).join(name);
                        let src = if src.starts_with("/") {
                            src.to_string_lossy().to_string()
                        } else {
                            format!("/{}", src.to_string_lossy())
                        };

                        let link = format!("<a href=\"{}\">{}</a>", src, name);
                        format!("<li>{}{}</li>", if *is_dir { "(dir) - " } else { "" }, link)
                    })
                    .collect::<Vec<_>>()
                    .join("");

                let html = format!(
                    r#"
                    <!DOCTYPE html>
                    <html>
                        <head>
                            <title>Index of {path}</title>
                        </head>
                        <body>
                            <h1>Index of {path}</h1>
                            <ul>
                                {html}
                            </ul>
                        </body>
                    </html>
                    "#,
                    path = path.display(),
                    html = html
                );
                headers.insert("Content-Type", "text/html".parse().unwrap());
                (StatusCode::OK, headers, html)
            }
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, headers, e.to_string()),
        }
    } else if !path.exists() {
        (StatusCode::NOT_FOUND, headers, "Not Found".to_string())
    } else {
        match fs::read_to_string(path).await {
            Ok(content) => (StatusCode::OK, headers, content),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, headers, e.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(AppState {
            dir: PathBuf::from("."),
        });

        let (status, _, content) =
            file_handler(State(state.clone()), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.starts_with("[package]"));

        let (status, _, content) =
            file_handler(State(state.clone()), Path("not_found.txt".to_string())).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(content, "Not Found");
    }

    #[tokio::test]
    async fn test_static_server_200() {
        let response = reqwest::get("http://localhost:8080/assets/juventus.csv").await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_static_server_404() {
        let response = reqwest::get("http://localhost:8080/assets/juventus1.csv").await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_tower_http() {
        let response = reqwest::get("http://localhost:8080/tower/assets/juventus.csv").await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}