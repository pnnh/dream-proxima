use crate::config::is_debug;
use async_graphql::futures_util::TryFutureExt;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_appconfig::Client;

pub async fn get_config() -> Result<String, String> {
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
    let response = request.send().await.map_err(|err| err.to_string())?;

    if let Some(blob) = response.content() {
        let data = blob.clone().into_inner();
        let content = String::from_utf8(data).map_err(|err| err.to_string())?;
        tracing::debug!("获取到配置\n{}", content);
        return Ok(content);
    }
    Err("出错".to_string())
}
