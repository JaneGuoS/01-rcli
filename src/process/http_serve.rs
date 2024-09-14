use std::{path::PathBuf, sync::Arc};
use axum::{extract::{Path, State}, http::StatusCode, routing::get, Router};
use std::net::SocketAddr;
use tracing::{info, warn};
use anyhow::Result;

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()>{
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serveing {:?} on port {}", path, port);
    let state = HttpServeState { path };
    //axum router
    let router  = Router::new()
        .route("/*path", get(file_handler)).with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}


async fn file_handler(State(state):
State<Arc<HttpServeState>>, Path(path): Path<String>) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Requesting file: {:?}", p);
    if !p.exists() {
        (StatusCode::NOT_FOUND, format!("File not found: {:?}", p))
    }
    else {
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Reading {} bytes", content.len());
                (StatusCode::OK, content)
            }
            Err(e) => {
                warn!("Error reading file: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}
