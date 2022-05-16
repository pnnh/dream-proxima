use crate::config;

pub fn get_photo_or_default(photo_path: &str) -> String {
    if !photo_path.is_empty() {
        let mut file_url = "".to_string();
        file_url.push_str(config::FILE_URL);
        file_url.push_str(photo_path);
        return file_url;
    }
    config::DEFAULT_FILE_URL.to_string()
}
