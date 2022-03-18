use std::error::Error;
use std::time::Duration;

use json::{self, JsonValue};
use log::*;
use reqwest::blocking::{Client, Response};

use super::trigger::Trigger;
use super::trigger::TriggerInfo;

pub struct DiscordNotification {
    client: Client,
    webhook_url: String,
}

impl DiscordNotification {
    pub fn new(webhook_url: &str) -> Self {
        DiscordNotification {
            client: Client::new(),
            webhook_url: String::from(webhook_url),
        }
    }
}

impl Trigger for DiscordNotification {
    fn pull_trigger(&self, trigger_info: &TriggerInfo) -> Result<(), Box<dyn Error>> {
        debug!("DiscordNotification start: {:?}", trigger_info);

        let content = format!(
            "--------------------\nTitle: {}\nLink: {}\nDate: {}",
            trigger_info.get_title().as_ref().unwrap_or(&String::new()),
            trigger_info.get_link().as_ref().unwrap_or(&String::new()),
            trigger_info
                .get_pub_date()
                .as_ref()
                .unwrap_or(&String::new()),
        );

        let json: JsonValue = json::object! {
            wait: true,
            content: content
        };

        let res: Response = self
            .client
            .post(&self.webhook_url)
            .body(json.dump())
            .header("Content-Type", "application/json")
            .timeout(Duration::from_millis(10000))
            .send()?;

        if res.status().is_success() {
            debug!("DiscordNotification success");
            Ok(())
        } else {
            debug!("DiscordNotification failed: {}", res.text()?);
            Err("")?
        }
    }
}
