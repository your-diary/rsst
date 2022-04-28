#[derive(Debug)]
pub struct FeedConfig {
    url: String,
    pub should_omit_summary_from_atom_hash: bool,
    pub should_omit_content_from_atom_hash: bool,
    pub should_omit_updated_from_atom_hash: bool,
    pub should_omit_pub_date_from_rss_hash: bool,
    pub is_golang_blog_mode: bool,
}

impl FeedConfig {
    pub fn new(url: &str) -> Self {
        FeedConfig {
            url: url.to_string(),
            should_omit_summary_from_atom_hash: true,
            should_omit_content_from_atom_hash: true,
            should_omit_updated_from_atom_hash: false,
            should_omit_pub_date_from_rss_hash: false,
            is_golang_blog_mode: false,
        }
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }
}
