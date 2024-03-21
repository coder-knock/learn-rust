use toml::Table;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn main() {
    read_toml();
    write_toml();
}

#[derive(Deserialize, Debug)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct TomlConfig {
    package: Package,
    dependencies: HashMap<String, String>,
}

fn read_toml() {
    let toml_file = std::env::args().nth(1).expect("no file given");
    let toml_file_content = std::fs::read_to_string(toml_file).expect("failed to read file");
    let toml_value = toml_file_content.parse::<Table>().expect("failed to parse TOML");
    println!("{:#?}", toml_value);
    for x in &toml_value {
        println!("key {} value {}", x.0, x.1);
    }
    println!("authors ==> {:?}", toml_value.get("package").unwrap().get("authors").unwrap().as_array().unwrap());

    let toml_config: TomlConfig = toml::from_str(&toml_file_content).unwrap();
    println!("toml_config ==> {:#?}", toml_config);
    println!("authors ==> {:?}", toml_config.package.authors.get(0).unwrap());
}

#[derive(Serialize)]
struct Config {
    ip: String,
    port: Option<u16>,
    keys: Keys,
}

#[derive(Serialize)]
struct Keys {
    github: String,
    travis: Option<String>,
}

fn write_toml() {
    let config = Config {
        ip: "127.0.0.1".to_string(),
        port: Option::from(8090),
        keys: Keys {
            github: "coderknock".to_string(),
            travis: Some("lear rust".to_string()),
        },
    };

    let toml = toml::to_string(&config).unwrap();
    std::fs::write("./data/config.toml", toml).unwrap();
}
