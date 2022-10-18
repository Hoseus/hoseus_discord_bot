use rand::prelude::SliceRandom;

const ANIMATIONS_FILE_NAME: &str = "animation_urls.json";

fn load_animation_urls() -> Vec<String> {
    let file_path = ANIMATIONS_FILE_NAME.to_string();
    let json_string: String = std::fs::read_to_string(file_path.to_string())
        .expect(format!("Error. Unable to read file {}", file_path).as_str());
    let animation_urls: Vec<String> = serde_json::from_str(json_string.as_str())
        .expect(format!("Error. Unable to parse json in {}", file_path).as_str());
    if animation_urls.is_empty() {
        panic!("Error. Animation urls list cannot be empty");
    }
    animation_urls
}

lazy_static! {
    static ref ANIMATION_URLS: Vec<String> = load_animation_urls();
}

pub fn get_animation_urls_size() -> usize {
    ANIMATION_URLS.len()
}

pub fn get_animation_urls() -> Vec<String> {
    ANIMATION_URLS.to_owned()
}

pub fn get_random_animation_url() -> String {
    let mut rng = rand::thread_rng();
    ANIMATION_URLS.choose(&mut rng).unwrap().to_string()
}

pub fn get_animation_url(index: usize) -> Option<String> {
    ANIMATION_URLS
        .get(index)
        .map(|animation_url| animation_url.to_owned())
}
