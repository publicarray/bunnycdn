#![allow(unused)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod bunnycdn_storage {
    use std::fs;

    #[derive(Debug)]
    struct StorageZone {
        name: String,
        api_key: String,
    }

    struct StorageObject {
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
        pub fn upload_file(&self, file_path: String, object_url: String) -> String {
            let contents =
                fs::read_to_string(file_path).expect("Something went wrong reading the file");
            //network PUT file contents
            //AccessKey: self.api_key
            "ok".to_string()
        }
        pub fn download_file(&self, file_path: String, object_url: String) -> String {
            //network GET file contents
            let contents =
                fs::write(file_path, "test").expect("Something went wrong writing the file");

            "ok/404".to_string()
        }

        pub fn get_objects(&self, directory_url: String) -> StorageObject {
            //network GET file contents
            StorageObject {
                guid: "".to_string(),
                user_id: "".to_string(),
                date_created: "".to_string(),
                last_changed: "".to_string(),
                storage_zone_name: "".to_string(),
                path: "".to_string(),
                object_name: "".to_string(),
                length: 0,
                is_directory: false,
                server_id: "".to_string(),
                storage_zone_id: "".to_string(),
                full_path: "".to_string(),
            }
        }

        pub fn delete_object(object_url: String) -> String {
            //network DELETE
            "unimplemented".to_string()
        }
    }
}
