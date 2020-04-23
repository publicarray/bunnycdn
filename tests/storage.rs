#[cfg(test)]

mod tests {
    use bunnycdn::storage::{ResponseData, StorageZone};
    use tokio::runtime::{Builder, Runtime};

    fn rt() -> Runtime {
        Builder::new().basic_scheduler().enable_all().build().unwrap()
    }

    fn sz() -> StorageZone {
        StorageZone::new("testfiles".to_string(), "".to_string())
            .set_api_endpoint("https://private-anon-b7dd339e69-bunnycdnstorage.apiary-mock.com")
    }

    #[test]
    fn create_storage_zone() {
        let sz = sz();
        assert_eq!(sz.name(), "testfiles".to_string());
    }

    #[test]
    fn get_objects() {
        let mut rt = rt();
        let sz = sz();

        // let so: StorageObject = rt.block_on(sz.get_objects("/")).unwrap();
        // println!("{:?}", so);
        let _response = rt.block_on(sz.get_objects("%2F")).unwrap();
        // assert!(StatusCode::OK.is_success());
    }

    #[test]
    fn get_objects_404() {
        let mut rt = rt();
        let sz = sz();

        let response = rt.block_on(sz.get_objects("")).unwrap();
        println!("{:?}", response);
        let status_code = match response {
            ResponseData::HttpStatus(reqwest::StatusCode::NOT_FOUND) => 404,
            _ => 1,
        };
        assert_eq!(status_code, 404);
    }

    #[test]
    fn upload() {
        let mut rt = rt();
        let sz = sz();

        let _statuscode = rt.block_on(sz.upload_file("tests/test.txt", "test.txt")).unwrap();
    }

    #[test]
    fn download() {
        let mut rt = rt();
        let sz = sz();

        let _statuscode = rt
            .block_on(sz.download_file("tests/300kb.jpg", "/testfiles/images/300kb.jpg"))
            .unwrap();
        // TODO assert return status
    }

    #[test]
    fn delete() {
        let mut rt = rt();
        let sz = sz();

        let _statuscode = rt.block_on(sz.delete("/testfiles/images/300kb.jpg")).unwrap();
        // TODO assert return status
    }
}
