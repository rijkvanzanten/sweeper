use crate::game::Board;
use crate::state::GamesState;
use axum::{debug_handler, extract::Form, extract::State, response::Redirect};
use serde::Deserialize;
use crate::utils::gen_id;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GameInput {
	size: usize,
	mines: usize,
}

#[debug_handler]
pub async fn post_new(State(games): State<GamesState>, Form(input): Form<GameInput>) -> Redirect {
	let new_game = Board::new(input.size, input.size, input.mines);
	let id = gen_id();

	let route = "/game/".to_string() + &id;
	games.write().unwrap().insert(id, new_game);

	Redirect::to(&route)
}
