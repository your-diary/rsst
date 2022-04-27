#[derive(Debug)]
pub struct FeedUrl {
    url: String,
}

impl FeedUrl {
    pub fn new(url: &str) -> Self {
        FeedUrl {
            url: url.to_string(),
        }
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }
}
