use log::*;
use std::env;
use std::fs;

use rsst;
use rsst::atom::Atom;
use rsst::atom::AtomEntry;
use rsst::database::Database;
use rsst::feedtype::FeedType;
use rsst::rss::Rss;
use rsst::rss::RssItem;

fn handle_rss_feed_case(db: &Database, contents: &str) {
    let rss = Rss::new(&contents);

    let parent_hash = rss.hash_code();

    if (!db.does_rss_feed_exist(&rss)) {
        debug!(
            "New site: {} / {} / {}",
            rss.hash_code(),
            rss.get_title(),
            rss.get_link()
        );
        db.insert_rss_feed(&rss);
        db.insert_rss_feed_items(&parent_hash, rss.get_item_list());
        debug!("Notify: {:?}", rss.get_item_list().first());
        //TODO: notify last element
    } else {
        debug!(
            "Existent site: {} / {} / {}",
            rss.hash_code(),
            rss.get_title(),
            rss.get_link()
        );
        let existent_rss_items: Vec<String> =
            db.select_rss_feed_items(&rss.get_item_list().iter().map(|e| e.hash_code()).collect());
        let mut new_rss_items: Vec<RssItem> = rss.get_item_list().clone();
        new_rss_items.retain(|e| !existent_rss_items.contains(&e.hash_code()));
        debug!("New rss items: {:?}", new_rss_items);
        db.insert_rss_feed_items(&parent_hash, &new_rss_items);
        debug!("Notify: {:?}", new_rss_items);
        //TODO: notify them
    }
}

fn handle_atom_feed_case(db: &Database, contents: &str) {
    let atom = Atom::new(&contents);

    let parent_hash = atom.hash_code();

    if (!db.does_atom_feed_exist(&atom)) {
        debug!(
            "New site: {} / {} / {}",
            atom.hash_code(),
            atom.get_title(),
            atom.get_id()
        );
        db.insert_atom_feed(&atom);
        db.insert_atom_feed_entries(&parent_hash, atom.get_entry_list());
        debug!("Notify: {:?}", atom.get_entry_list().first());
        //TODO: notify last element
    } else {
        debug!(
            "Existent site: {} / {} / {}",
            atom.hash_code(),
            atom.get_title(),
            atom.get_id()
        );
        let existent_atom_entries: Vec<String> = db.select_atom_feed_entries(
            &atom
                .get_entry_list()
                .iter()
                .map(|e| e.hash_code())
                .collect(),
        );
        let mut new_atom_entries: Vec<AtomEntry> = atom.get_entry_list().clone();
        new_atom_entries.retain(|e| !existent_atom_entries.contains(&e.hash_code()));
        debug!("New atom entries: {:?}", new_atom_entries);
        db.insert_atom_feed_entries(&parent_hash, &new_atom_entries);
        debug!("Notify: {:?}", new_atom_entries);
        //TODO: notify them
    }
}

fn main() {
    const SHOULD_LOG_DEBUG: bool = true;
    if (SHOULD_LOG_DEBUG) {
        env::set_var("RUST_LOG", "debug");
    } else {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let db = Database::new("test.sqlite3");

    let contents: String = fs::read_to_string("../samples/arch_linux.xml").unwrap();
    // let contents: String = fs::read_to_string("../samples/sample-rss-2.xml").unwrap();
    //         let contents: String = fs::read_to_string("../samples/alpine_linux.xml").unwrap();
    //     let contents: String = String::from("hello");

    match FeedType::new(&contents) {
        FeedType::Rss => {
            handle_rss_feed_case(&db, &contents);
        }
        FeedType::Atom => {
            handle_atom_feed_case(&db, &contents);
        }
        _ => {
            panic!("Unknown feed type.");
        }
    }
}
