use crate::state::MinijinjaState;
use crate::utils::render;
use axum::{extract::State, http::StatusCode, response::{IntoResponse, Html}};

pub async fn get_index(State(env): State<MinijinjaState>) -> Result<Html<String>, impl IntoResponse> {
	let Ok(output) = render(&env, "index.html", ()) else {
      return Err((StatusCode::INTERNAL_SERVER_ERROR, ()))
   };

   Ok(Html(output))
}
