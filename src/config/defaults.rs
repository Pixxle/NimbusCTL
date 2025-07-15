use crate::config::user_config::UserConfig;

pub fn get_default_config() -> UserConfig {
    UserConfig::default()
}

pub fn get_default_keybindings() -> Vec<(&'static str, &'static str)> {
    vec![("?", "Help")]
}
