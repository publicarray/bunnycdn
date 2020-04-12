#![allow(unused)]

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::error::Error;

const SERVER_URL: &str = "https://storage.bunnycdn.com";

#[derive(Debug)]
pub struct StorageZone {
    pub name: String,
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
pub struct GetResponse {
    http_code: u8,
    message: String,
}

impl StorageZone {
    pub fn new(name: String, api_key: String) -> Self {
        StorageZone { name, api_key }
    }

    pub async fn download_file(
        &self,
        file_path: &str,
        object_url: &str,
    ) -> Result<(), Box<dyn Error>> {
        let request_url = format!("{}/{}/{}", SERVER_URL, self.name, object_url);
        println!("{}", request_url);
        // todo do this in chunks/ don't put whole file into memory
        let response = reqwest::Client::new()
            .get(&request_url)
            .header("AccessKey", &self.api_key)
            .header("Accept", "application/json")
            .send()
            .await?;
        if response.status() == 200 {
            let data = response.text().await?;
            fs::write(file_path, data)?;
        } else {
            let data: GetResponse = response.json().await?;
            println!("{:?}", data);
        }
        Ok(())
    }

    pub async fn upload_file(
        &self,
        file_path: &str,
        object_url: &str,
    ) -> Result<reqwest::StatusCode, Box<dyn Error>> {
        let request_url = format!("{}/{}/{}", SERVER_URL, self.name, object_url);
        let pwd = env::current_dir().unwrap();
        println!(
            "request_url:{}, file_path:{}/{}",
            request_url,
            pwd.display(),
            file_path
        );
        let file_contents = fs::read(file_path)?;
        // todo do this in chunks/ don't put whole file into memory
        let response = reqwest::Client::new()
            .put(&request_url)
            .header("AccessKey", &self.api_key)
            .body(file_contents)
            .send()
            .await?;
        println!("{:?}", response.status());
        Ok(response.status())
    }

    pub async fn delete(&self, object_url: &str) -> Result<reqwest::StatusCode, reqwest::Error> {
        let request_url = format!("{}/{}/{}", SERVER_URL, self.name, object_url);
        println!("{}", request_url);

        let response = reqwest::Client::new()
            .delete(&request_url)
            .header("AccessKey", &self.api_key)
            .send()
            .await?;
        println!("{:?}", response.status());
        Ok(response.status())
    }

    pub async fn get_objects(&self, directory_url: &str) -> Result<(), reqwest::Error> {
        let request_url = format!("{}/{}/{}", SERVER_URL, self.name, directory_url);
        println!("{}", request_url);

        let response = reqwest::Client::new()
            .get(&request_url)
            .header("AccessKey", &self.api_key)
            .header("Accept", "application/json")
            .send()
            .await?;
        println!("response:\n\n{:?}", response.status());
        let status = response.status();
        if status == 200 {
            // {
            //     let data = &response.text().await?;
            //     println!("{:?}", data);
            // }
            let data: Vec<Option<StorageObject>> = response.json().await?;
            println!("{:?}", data);
        } else {
            let data = response.text().await?;
            println!("{} - {:?}", status, data);
        }

        Ok(())
    }
}
