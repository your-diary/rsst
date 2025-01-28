pub enum FeedType {
    Rss,
    Atom,
    Unknown,
}

impl FeedType {
    pub fn new(xml: &str) -> Self {
        if xml.contains("</rss>") {
            Self::Rss
        } else if xml.contains("</feed>") {
            Self::Atom
        } else {
            Self::Unknown
        }
    }
}
