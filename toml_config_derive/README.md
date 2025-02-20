# toml_config_derive
Basic rust derive macro and trait to turn a struct into a readable and writable TOML file.

[<img src="https://img.shields.io/crates/v/bottom.svg?style=flat-square" alt="crates.io">](https://crates.io/crates/toml_config_trait)
[<img src="https://img.shields.io/badge/docs-stable-66c2a5?style=flat-square&labelColor=555555&logoColor=white" alt="Docs">](https://docs.rs/toml_config_trait)


## Installation & Usage

```cargo add toml_config_derive```

```Rust
use toml_config_derive::TomlConfig;
use toml_config_derive::TomlConfigTrait;
```
Both the trait and the macro must be in scope to use the provided methods.

Structs using `TomlConfig` **MUST** also implement `Default`, `Serialize`, and `Deserialize`.

## Example Usage

```Rust
use toml_config_derive::{TomlConfig, TomlConfigTrait};
use serde::{Serialize, Deserialze};

#[derive(TomlConfig, Serialize, Deserialize, Default)]
TestStruct {
	first: String,
	second: usize,
}

fn main() {
	let test_struct = TestStruct::default();
	test_struct.write_to_path("test_config.toml".into()).unwrap();
	let test_struct_read = TestStruct::read_from_path("test_config.toml".into()).unwrap();
	assert_eq!(test_struct, test_struct_read);
}
```

## Features
`config_map` is used to convert a TOML config to a flat map that can be used for a Kubernetes config-map.

#### Example

```cargo add toml_config_derive -F config_map```


```Rust
use toml_config_derive::{TomlConfig, TomlConfigTrait};
use serde::{Serialize, Deserialze};

#[derive(TomlConfig, Serialize, Deserialize, Default)]
TestStruct {
	first: String,
	second: usize,
}

#[tokio::main]
async fn main() {
	let test_struct = TestStruct::read_from_path("test_config.toml".into()).unwrap();
	test_struct.update_config_map("test_config_map", "test_namespace").await.unwrap();
}
```

Under the hood this uses 
```
	let config = kube::Config::infer().await?;
	let client = kube::Client::try_from(config)?;
	let maps: kube::Api<k8s_openapi::api::core::v1::ConfigMap> = kube::Api::namespaced(client, namespace);
```
`k8s_openapi` is pinned to `v1_32`.

Note: this is pretty niche, but necessary for my use case. I'm open to ideas to make this more adaptable if there's a need for it.

