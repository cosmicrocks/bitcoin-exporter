use anyhow::{Result};
use serde::Deserialize;
use std::{env, default};

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

// default impl for Config
impl default::Default for Config {
	fn default() -> Self {
		Config {
			host: default_host(),
			user: "".to_owned(),
			password: "".to_owned(),
			bind: default_bind(),
		}
	}
}

impl Config {
	pub fn read() -> Result<Config> {

		let mut config: Config = Config::default();

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
