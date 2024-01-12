mod stream;
use serde::Deserialize;

use self::stream::data_bytes;

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
    let mut i: u32 = 1;
    let a = data_bytes(&mut i);
    println!("{:?}", a);
    Ok(data)
}
