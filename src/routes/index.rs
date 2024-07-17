use crate::render::render;
use axum::{extract::State, response::IntoResponse};
use minijinja::Environment;
use std::sync::Arc;

pub async fn get_index(env: State<Arc<Environment<'static>>>) -> impl IntoResponse {
	render(&env, "index.html", ())
}
