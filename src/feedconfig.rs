#[derive(Debug, Clone)]
pub struct FeedConfig {
    url: String,
    pub should_omit_date_field_from_hash: bool,
    pub is_golang_blog_mode: bool,
}

impl FeedConfig {
    pub fn new(url: &str) -> Self {
        FeedConfig {
            url: url.to_string(),
            should_omit_date_field_from_hash: false,
            is_golang_blog_mode: false,
        }
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }
}
