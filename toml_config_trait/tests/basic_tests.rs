use anyhow::Result;
use serde::{Deserialize, Serialize};
use toml_config_trait::{TomlConfig, TomlConfigTrait};

#[derive(TomlConfig, Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct TestConfig {
    pub field_one: usize,
    pub field_two: String,
    pub field_three: Vec<String>,
}

#[derive(TomlConfig, Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct NestedConfig {
    first: FirstConfig,
    more_stuff: Vec<usize>,
    and_more: String,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct FirstConfig {
    second: SecondConfig,
    second_stuff: usize,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct SecondConfig {
    val: usize,
}

#[test]
fn write_nested_config() -> Result<()> {
    let nested = NestedConfig::default();
    let nested_res = nested.write_to_path("test_data/test_config_4.toml".into());
    assert!(nested_res.is_ok());
    let nested_read = NestedConfig::read_from_path("test_data/test_config_4.toml".into())?;
    assert_eq!(nested_read, NestedConfig::default());

    Ok(())
}

#[test]
fn write_and_read() -> Result<()> {
    let test_config = TestConfig::default();
    test_config.write_to_path("test_data/test_config_3.toml".into())?;
    let read_config = TestConfig::read_from_path("test_data/test_config_3.toml".into())?;
    assert_eq!(test_config, read_config);
    Ok(())
}

#[test]
fn write_to_toml_file() -> Result<()> {
    let test_config = TestConfig::default();
    let write_res = test_config.write_to_path("test_data/test_config_2.toml".into());
    assert!(write_res.is_ok());
    Ok(())
}

#[test]
fn read_from_toml_file() -> Result<()> {
    let read_config = TestConfig::read_from_path("test_data/test_config_1.toml".into())?;
    let test_config = TestConfig {
        field_one: 47,
        field_two: "test".to_string(),
        field_three: vec![
            "okay".to_string(),
            "nice".to_string(),
            "another".to_string(),
        ],
    };
    assert_eq!(read_config, test_config);
    Ok(())
}
