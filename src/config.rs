use anyhow::{Context, Result};
use serde::Deserialize;
use std::env;
use std::fs::File;

fn default_host() -> String {
	"http://127.0.0.1:8332".to_owned()
}

fn default_bind() -> String {
	"127.0.0.1:3000".to_owned()
}

/// Config file
#[derive(Deserialize, Clone)]
pub struct Config {
	/// bitcoin rpc host
	#[serde(default = "default_host")]
	pub host: String,
	/// rpc user
	#[serde(default)]
	pub user: String,
	/// rpc password
	#[serde(default)]
	pub password: String,
	/// bind to addr:port
	#[serde(default = "default_bind")]
	pub bind: String,
}

impl Config {
	pub fn read(config: &str) -> Result<Config> {
		// Open configuration file
		let file = File::open(&config).with_context(|| format!("Can't open {}", &config))?;
		// Deserialize configuration
		let mut config: Config =
			serde_yaml::from_reader(file).with_context(|| format!("Can't read {}", &config))?;

		// Check if config values are set in environment variables
		if let Ok(host) = env::var("BITCOIN_RPC_HOST") {
			config.host = "http://".to_owned() + &host + ":8332";
		}
		if let Ok(user) = env::var("BITCOIN_RPC_USER") {
			config.user = user;
		}
		if let Ok(password) = env::var("BITCOIN_RPC_PASSWORD") {
			config.password = password;
		}

		Ok(config)
	}
}
