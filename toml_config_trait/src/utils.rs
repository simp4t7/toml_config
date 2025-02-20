use anyhow::Result;
use k8s_openapi::api::core::v1::ConfigMap;
use kube::core::ObjectMeta;
use std::collections::BTreeMap;
use toml::Table;

pub fn create_config_map(toml_str: &str, config_map_name: &str) -> Result<ConfigMap> {
    let config_map = toml_to_map(toml_str)?;
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

pub fn toml_to_map(toml_str: &str) -> Result<BTreeMap<String, String>> {
    let config: BTreeMap<String, Table> = toml::from_str(toml_str)?;
    let mut flat_map: BTreeMap<String, String> = BTreeMap::new();
    for table in config.values() {
        for (key, val) in table {
            flat_map.insert(key.clone(), val.to_string().trim_matches('"').to_string());
        }
    }
    Ok(flat_map)
}
