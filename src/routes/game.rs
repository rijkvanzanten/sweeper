use axum::extract::{Path, State};
use crate::state::GamesState;

pub async fn get_game(State(games_state): State<GamesState>, Path(id): Path<String>) -> &'static str {
   let games = games_state.read().unwrap();

   let game = games.get(&id);

   dbg!(game);

   "hello world"
}