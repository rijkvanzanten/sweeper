mod render;

use axum::{extract::State, response::IntoResponse, routing::get, Router};
use minijinja::{path_loader, Environment};
use std::sync::Arc;
use tokio::net::TcpListener;

use render::render;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let bind_address = format!("{}:{}", "127.0.0.1", 8080);
	let listener = TcpListener::bind(&bind_address).await?;
	println!("Listening on http://{}", bind_address);

	let mut minijinja_env = Environment::new();
	minijinja_env.set_loader(path_loader("templates"));

	let app = Router::new()
		.route("/", get(root))
		.with_state(Arc::new(minijinja_env));

	axum::serve(listener, app).await.unwrap();

	Ok(())
}

async fn root(env: State<Arc<Environment<'static>>>) -> impl IntoResponse {
	render(&env, "index.html", ())
}
