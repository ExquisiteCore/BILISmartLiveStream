// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use bilismartlivestream::get_danmu_info;
use tauri::Manager;

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
        websocket_url = format!("ws://{}:{}/sub", first_host.host, first_host.ws_port);
    };
    format!("{},{}", websocket_url, data.data.token)
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            //let window = app.windows();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, handle_send_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[tauri::command]
async fn handle_send_message(window: tauri::Window, message: String) {
    // Send a message to the WebSocket server here
    //write.send(Message::Text(message)).await.unwrap();
    //window.emit("messageSent", None).unwrap();
}
