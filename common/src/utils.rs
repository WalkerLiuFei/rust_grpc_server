use std::fs;

use serde::de::DeserializeOwned;
use toml;

pub fn read_config_from_file<T: DeserializeOwned>(file_path: &str) -> Result<T, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string(file_path)?;
    let config: T = toml::from_str(&config_str)?;
    Ok(config)
}


#[cfg(test)]
mod test {
    use std::fs;
    use std::io::Write;

    use serde_derive::{Deserialize, Serialize};

    use super::*;

    #[derive(Debug, Deserialize, Serialize)]
    struct TestConfigStruct {
        pub name: String,
        pub age: i32,
    }

    #[test]
    fn test_read_config_from_file() -> Result<(), Box<dyn std::error::Error>> {
        let v = TestConfigStruct {
            name: "test_name".to_string(),
            age: 1,
        };
        let tmp_file_path = "./tmp.toml";
        let mut file = std::fs::File::create(&tmp_file_path)?;
        write!(file, "{}", toml::to_string(&v).unwrap())?;
        let config = read_config_from_file::<TestConfigStruct>(tmp_file_path)?;
        assert_eq!(config.name, "test_name");
        assert_eq!(config.age, 1);
        fs::remove_file(tmp_file_path)?;
        Ok(())
    }
}
