use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub emojis: bool,
    pub backticks: bool,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            emojis: true,
            backticks: true,
        }
    }
}

pub fn load_config() -> Result<Config, confy::ConfyError> {
    let cfg = confy::load("worktodo")?;
    Ok(cfg)
}
