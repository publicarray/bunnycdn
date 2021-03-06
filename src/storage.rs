#![allow(unused)]
// #![deny(missing_docs)]
use crate::serde_types::*;
use anyhow::{Context, Result};
// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
// use chrono::NativeDateTime;
use std::env;
// use std::error::Error;
use std::fs;
use super::APP_USER_AGENT;

const SERVER_URL: &str = "https://storage.bunnycdn.com";
const PROXY: &str = "http://127.0.0.1:8866";

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
                let json = serde_json::to_string_pretty(&storage).unwrap();
                println!("{}", json);
            }
            ResponseData::HttpStatus(status) => {
                if !status.is_success() {
                    error!("{}", status);
                } else {
                    println!("{}", status);
                }
            }
            ResponseData::BunnyStatus(status) => {
                println!("{:?}", status);
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageZone {
    api_endpoint: String,
    name: String,
    api_key: String,
}

use reqwest::header;
impl StorageZone {
    pub fn new(name: String, api_key: String) -> Self {
        StorageZone {
            name,
            api_key,
            api_endpoint: SERVER_URL.to_string(),
        }
    }

    pub fn set_api_endpoint(&mut self, api_endpoint: &str) -> &Self {
        self.api_endpoint = api_endpoint.to_string();
        self
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// convenience function - prepare request with common http headers, user-agent & proxy
    fn client(&self) -> Result<reqwest::Client> {
        let mut headers = header::HeaderMap::new();
        // headers.insert(header::ACCEPT_ENCODING, header::HeaderValue::from_static("gzip, br")); // already set
        let api_key = self.api_key.clone();
        headers.insert("AccessKey", header::HeaderValue::from_str(&api_key).unwrap());

        Ok(reqwest::Client::builder()
            // .proxy(reqwest::Proxy::all(PROXY)?)
            .user_agent(APP_USER_AGENT)
            .default_headers(headers)
            .build()?)
    }

    pub async fn download_file(&self, file_path: &str, object_url: &str) -> Result<ResponseData> {
        let request_url = format!("{}/{}/{}", self.api_endpoint, self.name, object_url);
        trace!("{}", request_url);
        // todo do this in chunks/ don't put whole file into memory
        let response = self.client()?
            .get(&request_url)
            .header("Accept", "application/json")
            .send()
            .await?;

        let http_status = response.status();
        let response_data = ResponseData::HttpStatus(http_status);
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

    pub async fn upload_file(&self, file_path: &str, object_url: &str) -> Result<ResponseData> {
        let request_url = format!("{}/{}/{}", self.api_endpoint, self.name, object_url);
        let pwd = env::current_dir().unwrap();
        trace!("request_url:{}, file_path:{}/{}", request_url, pwd.display(), file_path);
        let file_contents = fs::read(file_path)?;
        // todo do this in chunks/ don't put whole file into memory
        let response = self.client()?
            .put(&request_url)
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

    pub async fn delete(&self, object_url: &str) -> Result<ResponseData> {
        let request_url = format!("{}/{}/{}", self.api_endpoint, self.name, object_url);
        trace!("{}", request_url);

        let response = self.client()?
            .delete(&request_url)
            .send()
            .await?;

        let response_data = ResponseData::HttpStatus(response.status());
        // response_data.canonical_reason()
        // let json_response = BunnyResponse {http_code:http_status.as_u16(), Some(message:http_status.canonical_reason()).to_string()};

        // info!("{:?}", response_data.HttpStatus.as_u16());
        Ok(response_data)
    }

    pub async fn get_objects(&self, directory_url: &str) -> Result<ResponseData> {
        let request_url = format!("{}/{}/{}", self.api_endpoint, self.name, directory_url);
        trace!("{:?}", request_url);

        let response = self.client()?
            .get(&request_url)
            .header("Accept", "application/json")
            .send()
            .await?;

        let http_status = response.status();
        trace!("{}", http_status);
        // println!("{}", http_status);

        // let mut data = ResponseData::BunnyStatus(BunnyResponse {http_code:http_status.as_u16(), ..Default::default()});
        let mut response_data = ResponseData::HttpStatus(http_status);
        // let json_response = BunnyResponse {http_code:http_status.as_u16(), message:"".to_string()};
        if http_status.as_u16() == 200 {
            // let data: Vec<Option<StorageObject>> =
            //     response.json().await.expect("Can't parse JSON! Make sure to select a directory not a file!");
            let data = response.text().await?;
            trace!("{:?}", data);
            // println!("{:?}", data);
            let data = serde_json::from_str::<Vec<Option<StorageObject>>>(&data)
                .context("Can't parse JSON! Make sure to select a directory not a file")?;
            trace!("{:?}", data);
            // println!("{:?}", data);
            response_data = ResponseData::StorageInfo(data);
            trace!("{:?}", response_data);
        } else if http_status.as_u16() == 404 {
            response_data = match response.json().await {
                Ok(data) => ResponseData::BunnyStatus(data), // return json if there is any
                Err(_) => response_data,                     // return HTTP status code
            };
            trace!("{:?}", response_data);
        } else {
            let data = response.text().await?;
            trace!("{} - {:?}", http_status, data);
        }
        Ok(response_data)
    }
}
