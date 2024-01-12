use serde_json::json;

pub fn data_bytes(index: &mut u32) -> Vec<u8> {
    let room_id = "870691";
    let data = json!({ "roomid": room_id });
    let data_str = serde_json::to_string(&data).unwrap();
    let mut data_bytes = Vec::new();

    data_bytes.extend_from_slice(&(0x0010 + data_str.len() as u32).to_be_bytes());
    data_bytes.extend_from_slice(b"\x00\x10");
    data_bytes.extend_from_slice(&(1u16).to_be_bytes());
    data_bytes.extend_from_slice(&(7u32).to_be_bytes());
    data_bytes.extend_from_slice(&index.to_be_bytes());

    data_bytes.extend_from_slice(data_str.as_bytes());
    data_bytes
}
