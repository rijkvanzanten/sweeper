use crate::game::Game;
use crate::state::GamesState;
use crate::utils::gen_id;
use axum::{debug_handler, extract::Form, extract::State, response::Redirect};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GameInput {
	size: u8,
	mines: u16,
}

#[debug_handler]
pub async fn post_new(State(games): State<GamesState>, Form(input): Form<GameInput>) -> Redirect {
	let id = gen_id();
	let route = "/".to_string() + &id.clone();

	let new_game = Game::new(input.size, input.mines);

	games.write().unwrap().insert(id, new_game);

	Redirect::to(&route)
}
