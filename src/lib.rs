pub mod atom;
pub mod command;
pub mod config;
pub mod database;
pub mod discord;
pub mod feedconfig;
pub mod feedtype;
pub mod rss;
pub mod trigger;
pub mod twitter;

use std::env;
use std::time::Duration;

use log::*;
use reqwest::blocking::Client;

use atom::Atom;
use atom::AtomEntry;
use database::Database;
use feedconfig::FeedConfig;
use rss::Rss;
use rss::RssItem;
use trigger::Trigger;
use trigger::TriggerInfo;

pub fn initialize_logger(should_log_debug: bool) {
    if (should_log_debug) {
        env::set_var("RUST_LOG", "debug");
    } else {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
}

pub fn retrieve_xml(url: &str) -> String {
    Client::new()
        .get(url)
        .timeout(Duration::from_millis(10000))
        .send()
        .unwrap()
        .text()
        .unwrap()
}

pub fn handle_rss_feed_case(
    db: &Database,
    contents: &str,
    trigger_list: &Vec<Box<dyn Trigger>>,
    feed_config: &FeedConfig,
) {
    let rss = Rss::new(contents, feed_config);

    let parent_hash = rss.hash_code();

    if (!db.does_feed_exist(&rss.hash_code())) {
        debug!(
            "New site: {} / {} / {}",
            rss.hash_code(),
            rss.get_title(),
            rss.get_link()
        );

        let latest_feed_item = rss.get_item_list().first().unwrap();

        //To confirm that the triggers successfully work for the new site,
        // we pull each trigger only for the latest feed item.
        let is_trigger_success = trigger_list.iter().all(|e| {
            e.pull_trigger(&TriggerInfo::new(
                latest_feed_item.get_title(),
                latest_feed_item.get_link(),
                latest_feed_item.get_description(),
                latest_feed_item.get_pub_date(),
            ))
            .is_ok()
        });

        if (is_trigger_success) {
            db.insert_rss_feed(&rss);
            db.insert_rss_feed_items(&parent_hash, rss.get_item_list());
        }
    } else {
        debug!(
            "Existent site: {} / {} / {}",
            rss.hash_code(),
            rss.get_title(),
            rss.get_link()
        );

        let existent_rss_items: Vec<String> =
            db.select_feed_items(&rss.get_item_list().iter().map(|e| e.hash_code()).collect());
        let mut new_rss_items: Vec<RssItem> = rss.get_item_list().clone();
        new_rss_items.retain(|e| !existent_rss_items.contains(&e.hash_code()));

        debug!("New rss items: {:?}", new_rss_items);

        for new_rss_item in new_rss_items {
            let is_trigger_success = trigger_list.iter().all(|e| {
                e.pull_trigger(&TriggerInfo::new(
                    new_rss_item.get_title(),
                    new_rss_item.get_link(),
                    new_rss_item.get_description(),
                    new_rss_item.get_pub_date(),
                ))
                .is_ok()
            });

            if (is_trigger_success) {
                db.insert_rss_feed_items(&parent_hash, &vec![new_rss_item]);
            }
        }
    }
}

pub fn handle_atom_feed_case(
    db: &Database,
    contents: &str,
    trigger_list: &Vec<Box<dyn Trigger>>,
    feed_config: &FeedConfig,
) {
    let atom = Atom::new(contents, feed_config);

    let parent_hash = atom.hash_code();

    if (!db.does_feed_exist(&atom.hash_code())) {
        debug!(
            "New site: {} / {} / {}",
            atom.hash_code(),
            atom.get_title(),
            atom.get_id()
        );

        let latest_atom_entry = atom.get_entry_list().first().unwrap();

        //To confirm that the triggers successfully work for the new site,
        // we pull each trigger only for the latest feed item.
        let is_trigger_success = trigger_list.iter().all(|e| {
            e.pull_trigger(&TriggerInfo::new(
                &Some(latest_atom_entry.get_title().to_string()),
                &Some(latest_atom_entry.get_id().to_string()),
                latest_atom_entry.get_summary_or_content(),
                &Some(latest_atom_entry.get_updated().to_string()),
            ))
            .is_ok()
        });

        if (is_trigger_success) {
            db.insert_atom_feed(&atom);
            db.insert_atom_feed_entries(&parent_hash, atom.get_entry_list());
        }
    } else {
        debug!(
            "Existent site: {} / {} / {}",
            atom.hash_code(),
            atom.get_title(),
            atom.get_id()
        );

        let existent_atom_entries: Vec<String> = db.select_feed_items(
            &atom
                .get_entry_list()
                .iter()
                .map(|e| e.hash_code())
                .collect(),
        );
        let mut new_atom_entries: Vec<AtomEntry> = atom.get_entry_list().clone();
        new_atom_entries.retain(|e| !existent_atom_entries.contains(&e.hash_code()));

        debug!("New atom entries: {:?}", new_atom_entries);

        for new_atom_entry in new_atom_entries {
            let is_trigger_success = trigger_list.iter().all(|e| {
                e.pull_trigger(&TriggerInfo::new(
                    &Some(new_atom_entry.get_title().to_string()),
                    &Some(new_atom_entry.get_id().to_string()),
                    new_atom_entry.get_summary_or_content(),
                    &Some(new_atom_entry.get_updated().to_string()),
                ))
                .is_ok()
            });

            if (is_trigger_success) {
                db.insert_atom_feed_entries(&parent_hash, &vec![new_atom_entry]);
            }
        }
    }
}
