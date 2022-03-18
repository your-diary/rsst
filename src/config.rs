use std::fs;

use json;
use json::JsonValue;

use super::discord::DiscordNotification;
use super::trigger::Trigger;

pub struct Config {
    should_log_debug: bool,
    database_file: String,
    trigger_list: Vec<Box<dyn Trigger>>,
    feed_url_list: Vec<String>,
}

impl Config {
    pub fn get_should_log_debug(&self) -> &bool {
        &self.should_log_debug
    }

    pub fn get_database_file(&self) -> &str {
        &self.database_file
    }

    pub fn get_trigger_list(&self) -> &Vec<Box<dyn Trigger>> {
        &self.trigger_list
    }

    pub fn get_feed_url_list(&self) -> &Vec<String> {
        &self.feed_url_list
    }

    pub fn new(config_file: &str) -> Self {
        let mut ret = Config {
            should_log_debug: false,
            database_file: String::new(),
            trigger_list: Vec::new(),
            feed_url_list: Vec::new(),
        };

        let json_string: String = fs::read_to_string(config_file).unwrap();

        match json::parse(&json_string).unwrap() {
            JsonValue::Object(o) => {
                ret.should_log_debug = o.get("should_log_debug").unwrap().as_bool().unwrap();

                ret.database_file = o
                    .get("database_file")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string();

                match o.get("feed_url_list").unwrap() {
                    JsonValue::Array(v) => {
                        ret.feed_url_list =
                            v.iter().map(|e| e.as_str().unwrap().to_string()).collect()
                    }
                    _ => panic!(),
                };

                match o.get("triggers").unwrap() {
                    JsonValue::Object(o) => {
                        for field in o.iter() {
                            match field {
                                ("discord", o) => match o {
                                    JsonValue::Object(o) => {
                                        if (o.get("enabled").unwrap().as_bool().unwrap()) {
                                            ret.trigger_list.push(Box::new(
                                                DiscordNotification::new(
                                                    o.get("webhook_url").unwrap().as_str().unwrap(),
                                                ),
                                            ));
                                        }
                                    }
                                    _ => panic!(),
                                },
                                _ => panic!(),
                            }
                        }
                    }
                    _ => panic!(),
                };
            }
            _ => panic!(),
        }

        ret.validate();

        ret
    }

    fn validate(&self) -> () {
        assert!(!self.database_file.is_empty());
        assert!(!self.trigger_list.is_empty());
        assert!(!self.feed_url_list.is_empty());
    }
}
