use std::{default, env};

use super::configs::config::{Configs, VariableConfigs};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Environment {
    #[default]
    Local,
    Dev,
    Staging,
    Prod,
}

#[derive(Default)]
pub struct ConfigEnv {}

impl ConfigEnv {
    pub fn new() -> ConfigEnv {
        ConfigEnv::default()
    }

    pub async fn build<'c, T>(&mut self) -> Result<Configs<T>, ()>
    where
        T: VariableConfigs,
    {
        let env = env::var("RUST_ENV").unwrap_or_default();
        match env.as_str() {
            "dev" => Environment::Dev,
            _ => Environment::Local,
        }
    }
}
