// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use bilismartlivestream::get_danmu_info;

#[tauri::command]
async fn greet(id: String) -> String {
    // 发送 HTTP 请求
    let data = match get_danmu_info::get_danmu_info(&id).await {
        Ok(a) => a,
        Err(err) => return err.to_string(),
    };

    let mut websocket_url = String::new();
    if let Some(first_host) = data.data.host_list.get(0) {
        // 拼接 ws://Host:ws_port 的字符串
        websocket_url = format!("ws://{}:{}", first_host.host, first_host.ws_port);
    };

    format!("{}和{}", websocket_url, data.data.token)
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
