use crate::game::Game;
use axum::extract::FromRef;
use minijinja::{path_loader, Environment};
use std::{collections::HashMap, sync::{Arc, RwLock}};

#[derive(Clone)]
pub struct AppState {
	minijinja_env: MinijinjaState,
	games: GamesState,
}

impl AppState {
	pub fn new() -> AppState {
		let mut minijinja_env = Environment::new();
		minijinja_env.set_loader(path_loader("templates"));

		AppState {
			minijinja_env: Arc::new(minijinja_env),
			games: Arc::new(RwLock::new(HashMap::new())),
		}
	}
}

pub type MinijinjaState = Arc<Environment<'static>>;

impl FromRef<AppState> for MinijinjaState {
	fn from_ref(app_state: &AppState) -> MinijinjaState {
		app_state.minijinja_env.clone()
	}
}

pub type GamesState = Arc<RwLock<HashMap<String, Game>>>;

impl FromRef<AppState> for GamesState {
	fn from_ref(app_state: &AppState) -> GamesState {
		app_state.games.clone()
	}
}
