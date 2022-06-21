use crate::layers::ProximaError;
use async_graphql::futures_util::TryFutureExt;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_appconfig::Client;
use std::collections::HashMap;
use std::ops::Index;

pub const FILE_URL: &str = "https://file.sfx.xyz";
pub const DEFAULT_FILE_URL: &str = "https://res.sfx.xyz/images/default.png";

#[derive(Debug, Clone)]
pub struct ProximaConfig {
    configuration: String,
    pub dsn: String,
}

impl ProximaConfig {
    pub async fn init() -> Result<ProximaConfig, ProximaError> {
        let region_provider = RegionProviderChain::default_provider().or_else("ap-east-1");
        let config = aws_config::from_env().region(region_provider).load().await;

        let client = Client::new(&config);

        let mut request = client
            .get_configuration()
            .client_id("proxima")
            .application("sfx");
        if is_debug() {
            request = request.configuration("debug.config").environment("debug");
        } else {
            request = request
                .configuration("release.config")
                .environment("release");
        };
        let response = request
            .send()
            .await
            .map_err(|err| ProximaError::from_string(err.to_string()))?;

        if let Some(blob) = response.content() {
            let data = blob.clone().into_inner();
            let content = String::from_utf8(data)
                .map_err(|err| ProximaError::from_string(err.to_string()))?;
            //tracing::debug!("获取到配置\n{}", content);
            let mut config = ProximaConfig {
                configuration: content,
                dsn: "".to_string(),
            };
            config.parse_config();
            return Ok(config);
        }
        Err(ProximaError::new("出错"))
    }

    pub fn get_configuration(&self) -> String {
        return self.configuration.clone();
    }

    pub fn parse_config(&mut self) {
        let split = self.configuration.split("\n");
        let mut config_map: HashMap<String, String> = HashMap::new();

        for s in split {
            let index = s.find("=").unwrap_or(0);
            if index > 0 {
                config_map.insert(s[..index].to_string(), s[index + 1..].to_string());
                let key = s[..index].to_string();
                let value = s[index + 1..].to_string();
                match key.as_str() {
                    "DSN" => self.dsn = value,
                    _ => {}
                }
            }
        }
    }
}

pub fn mode() -> String {
    let machine_kind = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    machine_kind.to_string()
}

pub fn is_debug() -> bool {
    mode() == "debug"
}
