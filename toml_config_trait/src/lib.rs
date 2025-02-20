use anyhow::Result;
use serde::de::DeserializeOwned;
use std::{fs, path::PathBuf};

use serde::Serialize;

#[cfg(feature = "config_map")]
mod utils;

pub use toml_config_derive::TomlConfig;

pub trait TomlConfigTrait {
    fn new_default_config() -> Self
    where
        Self: Default,
    {
        Self::default()
    }
    fn read_from_path(path: PathBuf) -> Result<Self>
    where
        Self: Default + DeserializeOwned,
    {
        let config: Self = match path.exists() {
            true => {
                let config_string = fs::read_to_string(&path)?;
                let config: Self = toml::from_str(&config_string)?;
                config
            }
            false => Self::new_default_config(),
        };
        Ok(config as Self)
    }
    fn write_to_path(&self, path: PathBuf) -> Result<()>
    where
        Self: Serialize,
    {
        let config_string = toml::to_string(&self)?;
        fs::write(path, config_string.as_bytes())?;
        Ok(())
    }
    #[cfg(feature = "config_map")]
    #[allow(async_fn_in_trait)]
    async fn update_config_map(&self, config_map_name: &str, namespace: &str) -> Result<()>
    where
        Self: Serialize,
    {
        use crate::utils::create_config_map;
        use crate::utils::toml_to_map;
        use k8s_openapi::api::core::v1::ConfigMap;
        use kube::{api::PostParams, Api, Client, Config};

        let config = Config::infer().await?;
        let client = Client::try_from(config)?;
        let maps: Api<ConfigMap> = Api::namespaced(client, namespace);

        let config_map = maps.entry("config-map").await?;
        let toml_str = toml::to_string(&self).expect("toml::to_string error");
        config_map
            .and_modify(|cm| {
                if let Some(data) = &cm.data {
                    let config_map = toml_to_map(&toml_str).expect("error parsing evo_config.toml");
                    if &config_map != data {
                        println!("config-map doesn't match, updating now.");
                        cm.data = Some(config_map);
                    } else {
                        println!("config-map is unchanged, skipping update");
                    }
                }
            })
            .or_insert(|| {
                create_config_map(&toml_str, config_map_name).expect("error creating config-map")
            })
            .commit(&PostParams::default())
            .await?;
        Ok(())
    }
}
