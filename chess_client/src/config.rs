pub use toml::Value;
use toml::map::Map;

use std::fs::File;
use std::io::prelude::*;

lazy_static! {
  /// The globol config data, loaded from "Server.toml" at the path where the program is executed.
  pub static ref CONFIG: Value = {
    let config = load_config(String::from("Server.toml"));
    match config {
      Ok(config) => config,
      Err(_) => Value::String(format!(""))
    }
  };
}

/// Load config data from the designated file.
fn load_config(path: String) -> Result<Value, std::io::Error> {
  let mut config_file = File::open(path)?;
  let mut content = String::new();

  config_file.read_to_string(&mut content)?;

  Ok(content.parse::<Value>()?)
}

/// Read the url from the config.
pub fn server_url() -> String {
  self::CONFIG.get("server")
    .unwrap_or(&Value::Boolean(false))
    .as_table()
    .unwrap_or(&Map::new())
    .get("url")
    .unwrap_or(&Value::String("127.0.0.1:3058".into()))
    .as_str().unwrap().into()
}