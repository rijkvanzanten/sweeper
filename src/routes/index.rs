use crate::state::MinijinjaState;
use crate::utils::render;
use axum::{extract::State, response::IntoResponse};

pub async fn get_index(State(env): State<MinijinjaState>) -> impl IntoResponse {
	render(&env, "index.html", ())
}
