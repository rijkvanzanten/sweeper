mod routes;
mod state;
mod utils;
mod game;

use axum::{
	routing::{get, post},
	Router,
};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let bind_address = format!("{}:{}", "127.0.0.1", 8080);
	let listener = TcpListener::bind(&bind_address).await?;
	println!("Listening on http://{}", bind_address);

	let app_state = state::AppState::new();

	let app = Router::new()
		.route("/", get(routes::get_index))
		.route("/new", post(routes::post_new))
		.route("/game/:game_id", get(routes::get_game))
      .route("/game/:game_id/flag", post(routes::post_flag))
		.nest_service("/assets", ServeDir::new("assets"))
		.with_state(app_state);

	axum::serve(listener, app).await.unwrap();

	Ok(())
}
