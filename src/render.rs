use axum::{
	http::StatusCode,
	response::{Html, IntoResponse},
};
use minijinja::Environment;
use serde::Serialize;

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
