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

        // curl --include --header "Accept: application/json" 'https://private-anon-b7dd339e69-bunnycdnstorage.apiary-mock.com/testfiles/%2F/'
        let response = rt.block_on(sz.get_objects("%2F")).unwrap();

        let data = match response {
            ResponseData::StorageInfo(data) => data,
            _ => Vec::new(),
        };

        if let Some(d) = data.get(0).unwrap() {
            if let Some(guid) = &d.guid {
                assert_eq!(guid, "d6445d80-a797-4535-bf0e-d3819bcdf928");
                return;
            }
        }
        assert!(false);
    }

    #[test]
    fn get_objects_404() {
        let mut rt = rt();
        let sz = sz();
        // curl --include --header "Accept: application/json" 'https://private-anon-b7dd339e69-bunnycdnstorage.apiary-mock.com/testfiles
        let response = rt.block_on(sz.get_objects("")).unwrap();
        let status_code = match response {
            ResponseData::HttpStatus(hs) => hs,
            _ => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        };
        assert_eq!(status_code.as_u16(), 404);
    }

    #[test]
    fn upload() {
        let mut rt = rt();
        let sz = sz();

        let response = rt.block_on(sz.upload_file("tests/test.txt", "test.txt")).unwrap();
        let status_code = match response {
            ResponseData::HttpStatus(hs) => hs,
            _ => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        };
        assert_eq!(status_code.as_u16(), 201); // upload successful
    }

    #[test]
    fn download() {
        let mut rt = rt();
        let sz = sz();

        let response = rt
            .block_on(sz.download_file("tests/300kb.jpg", "/images/300kb.jpg"))
            .unwrap();
        let status_code = match response {
            ResponseData::HttpStatus(hs) => hs,
            _ => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        };
        assert_eq!(status_code.as_u16(), 200);
    }

    #[test]
    fn delete() {
        let mut rt = rt();
        let sz = sz();

        let response = rt.block_on(sz.delete("/images/300kb.jpg")).unwrap();
        let status_code = match response {
            ResponseData::HttpStatus(hs) => hs,
            _ => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        };
        assert_eq!(status_code.as_u16(), 200);
    }
}
