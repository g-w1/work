use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub emojis: bool,
    pub backticks: bool,
    pub verbose: bool,
    pub show_id_in_ls: bool,
    pub ask_for_confirm: bool,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            emojis: true,
            backticks: true,
            verbose: true,
            show_id_in_ls: true,
            ask_for_confirm: true,
        }
    }
}

pub fn load_config() -> Result<Config, confy::ConfyError> {
    let cfg = confy::load("worktodo")?;
    Ok(cfg)
}
