use std::error::Error;

pub trait Trigger {
    fn pull_trigger(&self, trigger_info: &TriggerInfo) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug)]
pub struct TriggerInfo {
    title: Option<String>,
    link: Option<String>,
    description: Option<String>,
    pub_date: Option<String>,
}

impl TriggerInfo {
    pub fn new(
        title: &Option<String>,
        link: &Option<String>,
        description: &Option<String>,
        pub_date: &Option<String>,
    ) -> Self {
        TriggerInfo {
            title: title.clone(),
            link: link.clone(),
            description: description.clone(),
            pub_date: pub_date.clone(),
        }
    }

    pub fn get_title(&self) -> &Option<String> {
        &self.title
    }

    pub fn get_link(&self) -> &Option<String> {
        &self.link
    }

    pub fn get_description(&self) -> &Option<String> {
        &self.description
    }

    pub fn get_pub_date(&self) -> &Option<String> {
        &self.pub_date
    }
}
