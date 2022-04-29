use crate::{configuration, shared};
use shared::RequestSpecification;
use std::{fs, io::Write};

pub trait Cacher<T> {
    fn lookup(&self, request: &RequestSpecification) -> Option<T>;
    fn overwrite(&self, request: &RequestSpecification, storage: &T);
    fn append(&self, request: &RequestSpecification, storage: &T);
}

pub struct FileBasedCacher {
    configuration: configuration::Configuration,
}

impl Cacher<String> for FileBasedCacher {
    fn lookup(&self, request: &RequestSpecification) -> Option<String> {
        let cached_result_path = self.caching_strategy(request);
        match fs::read_to_string(cached_result_path) {
            Ok(stored_result) => Some(stored_result),
            Err(err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    None
                } else {
                    panic!("{:?}", err)
                }
            }
        }
    }

    fn overwrite(&self, request: &RequestSpecification, storage: &String) {
        let storage_path = self.caching_strategy(request);
        fs::create_dir_all(&storage_path.parent().unwrap()).unwrap();
        fs::write(storage_path, storage).unwrap();
    }

    fn append(&self, request: &RequestSpecification, storage: &String) {
        let storage_path = self.caching_strategy(request);
        fs::create_dir_all(&storage_path.parent().unwrap()).unwrap();
        let mut cache_file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(storage_path)
            .unwrap();
        cache_file.write_all(storage.as_bytes()).unwrap();
    }
}

impl FileBasedCacher {
    pub fn new(config: &configuration::Configuration) -> Self {
        FileBasedCacher {
            configuration: config.clone(),
        }
    }

    fn caching_strategy(&self, request: &RequestSpecification) -> std::path::PathBuf {
        let cache_dir = &self.configuration.cache_dir;
        let file_path: std::path::PathBuf = [
            cache_dir.to_str().unwrap(),
            &request.date.year.to_string(),
            &request.date.day.to_string(),
            &request.request_type.to_string(),
        ]
        .iter()
        .collect();

        file_path
    }
}
