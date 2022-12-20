use console::style;
use dialoguer::{Confirm, Input, Select};
use label_logger::{log, LabelTheme};
use std::{error::Error, net::IpAddr};

#[allow(dead_code)]
#[derive(Debug)]
struct Config {
	interface: IpAddr,
	hostname: String,
	use_acme: bool,
	private_key: Option<String>,
	cert: Option<String>,
}

fn init_config() -> Result<Option<Config>, Box<dyn Error>> {
	let theme = LabelTheme::default();

	log!("{} to the great setup wizard", style("Welcome").green());

	if !Confirm::with_theme(&theme)
		.with_prompt("Do you want to continue?")
		.default(true)
		.interact()?
	{
		return Ok(None);
	}

	let interface = Input::with_theme(&theme)
		.with_prompt("Interface")
		.default("127.0.0.1".parse().unwrap())
		.interact()?;

	let hostname = Input::with_theme(&theme)
		.with_prompt("Hostname")
		.interact()?;

	let tls = Select::with_theme(&theme)
		.with_prompt("Configure TLS")
		.default(0)
		.item("Automatic (with ACME)")
		.item("Manual")
		.item("No")
		.interact()?;

	let (use_acme, private_key, cert) = match tls {
		0 => (true, Some("acme.pkey".into()), Some("acme.cert".into())),
		1 => (
			false,
			Some(
				Input::with_theme(&theme)
					.with_prompt("├ Path to private key")
					.default("acme.pkey".into())
					.interact()?,
			),
			Some(
				Input::with_theme(&theme)
					.with_prompt("└ Path to certificate")
					.default("acme.cert".into())
					.interact()?,
			),
		),
		_ => (false, None, None),
	};

	Ok(Some(Config {
		hostname,
		interface,
		use_acme,
		private_key,
		cert,
	}))
}

fn main() {
	match init_config() {
		Ok(None) => println!("Aborted."),
		Ok(Some(config)) => println!("{config:#?}"),
		Err(err) => println!("error: {err}"),
	}
}
