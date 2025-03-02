use crate::state::AppState;
use crate::utils::render;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::IntoResponse;
use minijinja::context;

pub async fn get_game(
	State(state): State<AppState>,
	Path(id): Path<String>,
) -> Result<Html<String>, impl IntoResponse> {
	let games_state = state.games();
	let games = games_state.read().unwrap();

	let Some(game) = games.get(&id) else {
		return Err((StatusCode::NOT_FOUND, "Could not find game".to_owned()));
	};

   let rows = game.rows();

	let output = match render(
		&state.minijinja_env(),
		"game.j2",
		context! { rows => rows, width => game.width(), game_id => id },
	) {
		Ok(output) => output,
		Err(reason) => {
			return Err((StatusCode::INTERNAL_SERVER_ERROR, reason));
		}
	};

	Ok(Html(output))
}
