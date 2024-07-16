use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let bind_address = format!("{}:{}", "127.0.0.1", 8080);

	let listener = TcpListener::bind(&bind_address).await?;
	println!("Listening on http://{}", bind_address);

	let app = Router::new().route("/", get(root));

	axum::serve(listener, app).await.unwrap();

	Ok(())
}

async fn root() -> &'static str {
	"Hello, World!"
}
