use crate::game::Board;
use crate::state::GamesState;
use crate::utils::gen_id;
use axum::{extract::Form, extract::State, response::Redirect};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GameInput {
	width: usize,
	height: usize,
	mines: usize,
}

pub async fn post_new(State(games): State<GamesState>, Form(input): Form<GameInput>) -> Redirect {
	let new_game = Board::new(input.width, input.height, input.mines);
	let id = gen_id();

	let route = "/game/".to_string() + &id;
	games.write().unwrap().insert(id, new_game);

	Redirect::to(&route)
}
