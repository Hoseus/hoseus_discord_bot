use rand::prelude::SliceRandom;
use std::ops::Deref;
use std::sync::LazyLock;

const ANIMATIONS_FILE_NAME: &str = "animation_urls.json";
static ANIMATION_URLS: LazyLock<Vec<String>> = LazyLock::new(|| {
    let file_path = ANIMATIONS_FILE_NAME;
    let json_string: &str = &std::fs::read_to_string(file_path)
        .expect(format!("Error. Unable to read file {}", file_path).as_str());
    let animation_urls: Vec<String> = serde_json::from_str(json_string)
        .expect(format!("Error. Unable to parse json in {}", file_path).as_str());
    if animation_urls.is_empty() {
        panic!("Error. Animation urls list cannot be empty");
    }

    animation_urls
});

pub fn get_animation_urls_size() -> usize {
    ANIMATION_URLS.deref().len()
}

pub fn get_animation_urls() -> Vec<String> {
    ANIMATION_URLS.deref().to_owned()
}

pub fn get_random_animation_url() -> String {
    let mut rng = rand::thread_rng();
    ANIMATION_URLS.deref().choose(&mut rng).unwrap().to_string()
}

pub fn get_animation_url(index: usize) -> Option<String> {
    ANIMATION_URLS
        .deref()
        .get(index)
        .map(|animation_url| animation_url.to_owned())
}
