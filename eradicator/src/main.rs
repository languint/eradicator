use axum::routing::{Router, get};

#[tokio::main]
pub async fn main() -> Result<(), String> {
    let app: Router = Router::new().route("/", get(|| async { "eradicator" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .map_err(|e| format!("Failed to bind to port 3000: {e}"))?;

    axum::serve(listener, app)
        .await
        .map_err(|e| format!("Failed to serve app: {e}"))?;

    Ok(())
}
