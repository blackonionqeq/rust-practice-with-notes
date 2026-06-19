//! 当不使用json feature时，测试命令用cargo test --no-default-features
//! 当使用json feature时，测试命令用cargo test --features json
//! cargo test 测试时使用默认feature
//! cargo test --all-features 会测试所有feature

// 导入也能用这个语法
#[cfg(feature = "json")]
use anyhow::{Result, anyhow};

// 属性的条件编译
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct Config {
    // 属性也记得加上pub
    pub host: String,
    pub port: u16,
}

// 函数的条件编译，只有用了json feature才会编译
#[cfg(feature = "json")]
pub fn parse_json(data: &str) -> Result<Config> {
    serde_json::from_str(data).map_err(|e| anyhow!(e))
}

#[cfg(feature = "json")]
pub fn to_json(value: &serde_json::Value) -> Result<String> {
    serde_json::to_string(value).map_err(|e| anyhow!(e))
}

pub fn default_port() -> u16 {
    8080
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "json")]
    // 测试用例记得加上这个
    #[test]
    fn test_parse_json() {
        let data = r#"{"host": "localhost", "port": 8080}"#;
        let result = parse_json(data);
        assert!(result.is_ok());
        let config: Config = result.unwrap();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 8080);
    }

    #[cfg(feature = "json")]
    #[test]
    fn test_to_json() {
        let value = serde_json::json!({"host": "localhost", "port": 8080});
        let result = to_json(&value);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), r#"{"host":"localhost","port":8080}"#);
    }

    #[test]
    fn test_default_port() {
        assert_eq!(default_port(), 8080);
    }
}
