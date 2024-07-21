use axum::{
	http::StatusCode,
	response::{Html, IntoResponse},
};
use minijinja::Environment;
use rand::{distributions::Alphanumeric, Rng};
use serde::Serialize;

// TODO: try refactoring this to return anyhow results instead of IntoResponse, and relying on
// https://github.com/tokio-rs/axum/tree/main/examples/anyhow-error-response

pub fn render<S>(
	env: &Environment<'_>,
	name: &str,
	ctx: S,
) -> Result<Html<String>, impl IntoResponse>
where
	S: Serialize,
{
	let Ok(template) = env.get_template(name) else {
		return Err((
			StatusCode::INTERNAL_SERVER_ERROR,
			"Couldn't find MiniNinja template".to_owned(),
		));
	};

	let rendered = template.render(ctx).map_err(|err| {
		(
			StatusCode::INTERNAL_SERVER_ERROR,
			format!("Failed to render MiniNinja template: {err}"),
		)
	})?;

	Ok(Html(rendered))
}

pub fn gen_id() -> String {
	rand::thread_rng()
		.sample_iter(&Alphanumeric)
		.take(7)
		.map(char::from)
		.collect()
}
