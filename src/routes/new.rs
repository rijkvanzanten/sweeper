use crate::game::Game;
use crate::state::GamesState;
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
	let new_game = Game::new(input.size, input.mines);
	let id = new_game.id();

	let route = "/game/".to_string() + id;
	games.write().unwrap().insert(id.to_owned(), new_game);

	Redirect::to(&route)
}
