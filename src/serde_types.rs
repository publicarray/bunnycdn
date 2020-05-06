use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageZone {
    api_endpoint: String,
    name: String,
    api_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StorageObject {
    pub guid: Option<String>,
    pub user_id: Option<String>,
    // pub date_created: Option<DateTime::<Utc>>,
    // pub date_created: Option<NativeDateTime>,
    pub date_created: Option<String>,
    // pub last_changed: Option<DateTime::<Utc>>,
    // pub last_changed: Option<NativeDateTime>,
    pub last_changed: Option<String>,
    pub storage_zone_name: Option<String>,
    pub path: Option<String>,
    pub object_name: Option<String>,
    pub length: Option<usize>,
    pub is_directory: Option<bool>,
    pub server_id: Option<usize>,
    pub storage_zone_id: Option<usize>,
    pub checksum: Option<String>,
    pub replicated_zones: Option<String>,
    pub full_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub struct BunnyResponse {
    http_code: u16,
    message: String,
}
