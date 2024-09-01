use crate::state::GamesState;
use axum::extract::{Form, Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FlagInput {
	row: usize,
	col: usize,
}

pub async fn post_reveal(
	State(games): State<GamesState>,
	Path(id): Path<String>,
	Form(input): Form<FlagInput>,
) -> Result<Redirect, impl IntoResponse> {
	let mut games_state = games.write().unwrap();

	let Some(board) = games_state.get_mut(&id) else {
		return Err((StatusCode::NOT_FOUND, "Could not find game".to_owned()));
	};

	board.reveal(input.row, input.col);

	let route = "/game/".to_string() + &id;
	Ok(Redirect::to(&route))
}
