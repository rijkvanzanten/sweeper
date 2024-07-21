mod game;
mod routes;
mod state;
mod utils;

use axum::{
	routing::{get, post},
	Router,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let bind_address = format!("{}:{}", "127.0.0.1", 8080);
	let listener = TcpListener::bind(&bind_address).await?;
	println!("Listening on http://{}", bind_address);

	let app_state = state::AppState::new();

	let app = Router::new()
		.route("/", get(routes::get_index))
		.route("/new", post(routes::post_new))
		.with_state(app_state);

	axum::serve(listener, app).await.unwrap();

	Ok(())
}
