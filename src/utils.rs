use minijinja::{Environment, Value};
use rand::{distributions::Alphanumeric, Rng};

pub fn render(env: &Environment<'_>, name: &str, ctx: Value) -> Result<String, String> {
	let template = env.get_template(name).map_err(|err| format!("{}", err))?;
	let rendered = template.render(ctx).map_err(|err| format!("{}", err))?;

	Ok(rendered)
}

pub fn gen_id() -> String {
	rand::thread_rng()
		.sample_iter(&Alphanumeric)
		.take(7)
		.map(char::from)
		.collect()
}
