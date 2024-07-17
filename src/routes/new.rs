use axum::{extract::Form, response::Redirect};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GameInput {
	rows: u8,
	cols: u8,
}

// TODO add input validation to make sure cols/rows are within range

// TODO: custom error handling for extract failure:
// https://github.com/tokio-rs/axum/blob/main/examples/customize-extractor-error/src/main.rs

pub async fn post_new(Form(input): Form<GameInput>) -> Redirect {
	dbg!(input);
	Redirect::to("/")
}
