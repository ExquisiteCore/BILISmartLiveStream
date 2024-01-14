mod stream;

use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct HostInfo {
    pub host: String,
    pub port: u16,
    pub wss_port: u16,
    pub ws_port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub group: String,
    pub business_id: u32,
    pub refresh_row_factor: f64,
    pub refresh_rate: u32,
    pub max_delay: u32,
    pub token: String,
    pub host_list: Vec<HostInfo>,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub code: u32,
    pub message: String,
    pub ttl: u32,
    pub data: Data,
}

pub async fn get_danmu_info(id: &str) -> Result<ApiResponse, reqwest::Error> {
    let data: ApiResponse = reqwest::get(format!(
        "https://api.live.bilibili.com/xlive/web-room/v1/index/getDanmuInfo?id={}",
        id
    ))
    .await?
    .json()
    .await?;

    let mut websocket_url = String::new();
    if let Some(first_host) = data.data.host_list.get(0) {
        // 拼接 ws://Host:ws_port 的字符串
        websocket_url = format!("ws://{}:{}/sub", first_host.host, first_host.ws_port);
    };

    stream::ws_client(Url::parse("ws://zj-cn-live-comet.chat.bilibili.com:2244/sub").expect("msg"))
        .await;

    Ok(data)
}
