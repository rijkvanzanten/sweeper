use minijinja::{Environment, context};
use rand::{distributions::Alphanumeric, Rng};
use serde::Serialize;

pub fn render<S>(env: &Environment<'_>, name: &str, ctx: S) -> Result<String, String>
where
	S: Serialize,
{
	let Ok(template) = env.get_template(name) else {
		return Err(format!("Template {} not found", name));
	};

	let rendered = template.render(context!{ ctx }).map_err(|err| format!("{}", err))?;

	Ok(rendered)
}

pub fn gen_id() -> String {
	rand::thread_rng()
		.sample_iter(&Alphanumeric)
		.take(7)
		.map(char::from)
		.collect()
}
