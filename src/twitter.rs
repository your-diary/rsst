use std::error::Error;

use log::*;

use super::command;
use super::trigger::Trigger;
use super::trigger::TriggerInfo;

pub struct TwitterNotification;

const SCRIPT_PATH: &str = "./twitter/tweet.py";

impl TwitterNotification {
    pub fn new() -> Self {
        TwitterNotification {}
    }
}

impl Trigger for TwitterNotification {
    fn pull_trigger(&self, trigger_info: &TriggerInfo) -> Result<(), Box<dyn Error>> {
        debug!("TwitterNotification start: {:?}", trigger_info);

        let command = "python3";
        let args = vec![SCRIPT_PATH];
        let input = format!(
            "{}\n{}",
            trigger_info.get_title().as_ref().unwrap_or(&String::new()),
            trigger_info.get_link().as_ref().unwrap_or(&String::new()),
        );

        let result = command::run(command, &args, &input);

        if (result.exit_status == 0) {
            debug!("TwitterNotification success");
            Ok(())
        } else {
            debug!("TwitterNotification failed:");
            if (!result.stdout.is_empty()) {
                debug!("{}", result.stdout);
            }
            if (!result.stderr.is_empty()) {
                debug!("{}", result.stderr);
            }
            Err("")?
        }
    }
}
