#![allow(unused)]

extern crate reqwest;
use reqwest::multipart;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::fs;

const SERVER_URL: &str = "https://storage.bunnycdn.com";

#[derive(Debug)]
pub struct StorageZone {
    pub name: String,
    api_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageObject {
    guid: String,
    user_id: String,
    date_created: String,
    last_changed: String,
    storage_zone_name: String,
    path: String,
    object_name: String,
    length: usize,
    is_directory: bool,
    server_id: String,
    storage_zone_id: String,
    full_path: String,
}

impl StorageZone {
    pub fn new(name: String, api_key: String) -> Self {
        StorageZone { name, api_key }
    }

    pub async fn upload_file(
        &self,
        file_path: &str,
        object_url: &str,
    ) -> Result<(), reqwest::Error> {
        let request_url = format!("SERVER_URL/{}/{}", self.name, object_url);
        println!("{}", request_url);

        let file_contents = fs::read(file_path).expect("Something went wrong reading the file");
        let chunk = multipart::Part::bytes(file_contents);
        let form = multipart::Form::new().part("chunk", chunk);

        let response = reqwest::Client::new()
            .put(&request_url)
            .header("AccessKey", &self.api_key)
            .multipart(form)
            .send()
            .await?;
        Ok(())
    }

    pub async fn download_file(
        &self,
        file_path: &str,
        object_url: &str,
    ) -> Result<(), reqwest::Error> {
        let request_url = format!("SERVER_URL/{}/{}", self.name, object_url);
        println!("{}", request_url);

        let data = reqwest::Client::new()
            .get(&request_url)
            .header("AccessKey", &self.api_key)
            .send()
            .await?
            .text()
            .await?;
        fs::write(file_path, data).expect("Something went wrong writing the file");
        Ok(())
    }

    pub async fn get_objects(
        &self,
        directory_url: &str,
    ) -> Result<StorageObject, reqwest::Error> {
        let request_url = format!("SERVER_URL/{}/{}", self.name, directory_url);
        println!("{}", request_url);

        let storage_object: StorageObject = reqwest::Client::new()
            .get(&request_url)
            .header("AccessKey", &self.api_key)
            .send()
            .await?
            .json()
            .await?;
        println!("storage_object:\n\n{:?}", storage_object);
        Ok(storage_object)
    }

    pub async fn delete_object(
        &self,
        object_url: &str,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let request_url = format!("SERVER_URL/{}/{}", self.name, object_url);
        println!("{}", request_url);

        let response = reqwest::Client::new()
            .delete(&request_url)
            .header("AccessKey", &self.api_key)
            .send()
            .await?;
        Ok(response)
    }
}
