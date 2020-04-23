#![allow(unused)]
// #![deny(missing_docs)]
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs;

const SERVER_URL: &str = "https://storage.bunnycdn.com";

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageZone {
    api_endpoint: String,
    name: String,
    api_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StorageObject {
    guid: Option<String>,
    user_id: Option<String>,
    date_created: Option<NaiveDateTime>,
    last_changed: Option<NaiveDateTime>,
    storage_zone_name: Option<String>,
    path: Option<String>,
    object_name: Option<String>,
    length: Option<usize>,
    is_directory: Option<bool>,
    server_id: Option<usize>,
    storage_zone_id: Option<usize>,
    checksum: Option<String>,
    replicated_zones: Option<String>,
    full_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub struct BunnyResponse {
    http_code: u16,
    message: String,
}

#[derive(Debug)]
pub enum ResponseData {
    StorageInfo(Vec<Option<StorageObject>>),
    BunnyStatus(BunnyResponse),
    HttpStatus(reqwest::StatusCode),
}

impl ResponseData {
    pub fn print(&self) {
        match self {
            ResponseData::StorageInfo(storage) => {
                info!("{:?}", storage);
            }
            ResponseData::HttpStatus(status) => {
                info!("{}", status);
            }
            ResponseData::BunnyStatus(status) => {
                info!("{:?}", status);
            }
        }
    }
}

impl StorageZone {
    pub fn new(name: String, api_key: String) -> Self {
        StorageZone {
            name,
            api_key,
            api_endpoint: SERVER_URL.to_string(),
        }
    }

    pub fn set_api_endpoint(mut self, api_endpoint: &str) -> Self {
        self.api_endpoint = api_endpoint.to_string();
        self
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub async fn download_file(&self, file_path: &str, object_url: &str) -> Result<ResponseData, Box<dyn Error>> {
        let request_url = format!("{}/{}/{}", self.api_endpoint, self.name, object_url);
        trace!("{}", request_url);
        // todo do this in chunks/ don't put whole file into memory
        let response = reqwest::Client::new()
            .get(&request_url)
            .header("AccessKey", &self.api_key)
            .header("Accept", "application/json")
            .send()
            .await?;

        let http_status = response.status();
        let mut response_data = ResponseData::HttpStatus(http_status);
        if http_status.as_u16() == 200 {
            let data = response.text().await?;
            fs::write(file_path, data)?;
        }
        // Rely on http status codes than to phrase the json response. codes are the same
        // } else {
        //     info!("{:?}", http_status);
        //     let json_response: BunnyResponse = response.json().await?;
        //     response_data = ResponseData::BunnyStatus(json_response);
        // }
        Ok(response_data)
    }

    pub async fn upload_file(&self, file_path: &str, object_url: &str) -> Result<ResponseData, Box<dyn Error>> {
        let request_url = format!("{}/{}/{}", self.api_endpoint, self.name, object_url);
        let pwd = env::current_dir().unwrap();
        trace!("request_url:{}, file_path:{}/{}", request_url, pwd.display(), file_path);
        let file_contents = fs::read(file_path)?;
        // todo do this in chunks/ don't put whole file into memory
        let response = reqwest::Client::new()
            .put(&request_url)
            .header("AccessKey", &self.api_key)
            .body(file_contents)
            .send()
            .await?;

        let http_status = response.status();
        let response_data = ResponseData::HttpStatus(http_status);
        if http_status.as_u16() == 201 {
            info!("{:?}", "upload successful");
        }
        Ok(response_data)
    }

    pub async fn delete(&self, object_url: &str) -> Result<ResponseData, reqwest::Error> {
        let request_url = format!("{}/{}/{}", self.api_endpoint, self.name, object_url);
        trace!("{}", request_url);

        let response = reqwest::Client::new()
            .delete(&request_url)
            .header("AccessKey", &self.api_key)
            .send()
            .await?;

        let response_data = ResponseData::HttpStatus(response.status());
        // response_data.canonical_reason()
        // let json_response = BunnyResponse {http_code:http_status.as_u16(), Some(message:http_status.canonical_reason()).to_string()};

        // info!("{:?}", response_data.HttpStatus.as_u16());
        Ok(response_data)
    }

    pub async fn get_objects(&self, directory_url: &str) -> Result<ResponseData, reqwest::Error> {
        let request_url = format!("{}/{}/{}", self.api_endpoint, self.name, directory_url);
        trace!("{:?}", request_url);

        let response = reqwest::Client::new()
            .get(&request_url)
            .header("AccessKey", &self.api_key)
            .header("Accept", "application/json")
            .send()
            .await?;

        let http_status = response.status();
        // let mut data = ResponseData::BunnyStatus(BunnyResponse {http_code:http_status.as_u16(), ..Default::default()});
        let mut response_data = ResponseData::HttpStatus(http_status);
        // let json_response = BunnyResponse {http_code:http_status.as_u16(), message:"".to_string()};
        if http_status.as_u16() == 200 {
            let data: Vec<Option<StorageObject>> =
                response.json().await.expect("Please select a directory not a file!");
            // let data = response.text().await?;
            // println!("{:?}", data);
            // trace!("{:?}", data);
            response_data = ResponseData::StorageInfo(data);
            trace!("{:?}", response_data);
        } else if http_status.as_u16() == 404 {
            response_data = match response.json().await {
                Ok(data) => ResponseData::BunnyStatus(data), // return json if there is any
                Err(e) => response_data,                     // return HTTP status code
            };
            trace!("{:?}", response_data);
        } else {
            let data = response.text().await?;
            trace!("{} - {:?}", http_status, data);
        }
        Ok(response_data)
    }
}
