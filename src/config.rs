use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use json;
use json::JsonValue;
use regex::Regex;

use super::discord::DiscordNotification;
use super::feedconfig::FeedConfig;
use super::trigger::Trigger;
use super::twitter::TwitterNotification;

pub struct Config {
    should_log_debug: bool,
    database_file: String,
    trigger_list: Vec<Box<dyn Trigger>>,
    feed_config_list: Vec<FeedConfig>,
}

impl Config {
    pub fn new(config_file: &str) -> Self {
        let mut ret = Config {
            should_log_debug: false,
            database_file: String::new(),
            trigger_list: Vec::new(),
            feed_config_list: Vec::new(),
        };

        let json_string: String = {
            let file: File = File::open(config_file).unwrap();

            let comment_regex = Regex::new(r#"^\s*#.*"#).unwrap();

            BufReader::new(file)
                .lines()
                .filter(|l| !comment_regex.is_match(l.as_ref().unwrap()))
                .map(|l| l.unwrap())
                .collect::<Vec<String>>()
                .join("\n")
        };

        match json::parse(&json_string).unwrap() {
            JsonValue::Object(o) => {
                ret.should_log_debug = o.get("should_log_debug").unwrap().as_bool().unwrap();

                ret.database_file = o
                    .get("database_file")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string();

                match o.get("feed_config_list").unwrap() {
                    JsonValue::Array(v) => {
                        ret.feed_config_list = v
                            .iter()
                            .map(|o| match o {
                                JsonValue::Object(o) => {
                                    let mut feed_config =
                                        FeedConfig::new(o.get("url").unwrap().as_str().unwrap());
                                    if let Some(b) = o.get("should_omit_date_field_from_hash") {
                                        feed_config.should_omit_date_field_from_hash =
                                            b.as_bool().unwrap();
                                    }
                                    if let Some(b) = o.get("is_golang_blog_mode") {
                                        feed_config.is_golang_blog_mode = b.as_bool().unwrap();
                                    }
                                    feed_config
                                }
                                _ => panic!(),
                            })
                            .collect()
                    }
                    _ => panic!(),
                };

                match o.get("triggers").unwrap() {
                    JsonValue::Object(o) => {
                        for field in o.iter() {
                            match field {
                                ("discord", JsonValue::Object(o)) => {
                                    if o.get("enabled").unwrap().as_bool().unwrap() {
                                        ret.trigger_list.push(Box::new(DiscordNotification::new(
                                            o.get("webhook_url").unwrap().as_str().unwrap(),
                                        )));
                                    }
                                }
                                ("twitter", JsonValue::Object(o)) => {
                                    if o.get("enabled").unwrap().as_bool().unwrap() {
                                        ret.trigger_list.push(Box::new(TwitterNotification::new()));
                                    }
                                }
                                _ => panic!(),
                            }
                        }
                    }
                    _ => panic!(),
                };
            }
            _ => panic!(),
        }

        assert!(!ret.database_file.is_empty());
        assert!(!ret.trigger_list.is_empty());
        assert!(!ret.feed_config_list.is_empty());

        ret
    }

    pub fn get_should_log_debug(&self) -> &bool {
        &self.should_log_debug
    }

    pub fn get_database_file(&self) -> &str {
        &self.database_file
    }

    pub fn get_trigger_list(&self) -> &Vec<Box<dyn Trigger>> {
        &self.trigger_list
    }

    pub fn get_feed_config_list(&self) -> &Vec<FeedConfig> {
        &self.feed_config_list
    }
}
