use crate::state::MinijinjaState;
use crate::utils::render;
use axum::{extract::State, http::StatusCode, response::{IntoResponse, Html}};
use minijinja::context;

pub async fn get_index(State(env): State<MinijinjaState>) -> Result<Html<String>, impl IntoResponse> {
	let Ok(output) = render(&env, "index.html", context!{}) else {
      return Err((StatusCode::INTERNAL_SERVER_ERROR, ()))
   };

   Ok(Html(output))
}
