use anyhow::Result;
use k8s_openapi::api::core::v1::ConfigMap;
use kube::core::ObjectMeta;
use std::{collections::BTreeMap, fs};
use toml::Table;

pub fn create_config_map(config_path: &str, config_map_name: &str) -> Result<ConfigMap> {
    let config_map = toml_to_map(config_path)?;
    let metadata = ObjectMeta {
        name: Some(String::from(config_map_name)),
        ..Default::default()
    };
    let config_map = ConfigMap {
        data: Some(config_map),
        metadata,
        ..Default::default()
    };
    Ok(config_map)
}

pub fn toml_to_map(config_path: &str) -> Result<BTreeMap<String, String>> {
    let config: BTreeMap<String, Table> = toml::from_str(&fs::read_to_string(&config_path)?)?;
    let mut flat_map: BTreeMap<String, String> = BTreeMap::new();
    for table in config.values() {
        for (key, val) in table {
            flat_map.insert(key.clone(), val.to_string().trim_matches('"').to_string());
        }
    }
    Ok(flat_map)
}
