use log::*;

use rsst;
use rsst::config::Config;
use rsst::database::Database;
use rsst::feedtype::FeedType;
use rsst::trigger::Trigger;

const SHOULD_DROP_TABLES_FIRST: bool = false; //for debug

fn main() {
    let config = Config::new("./conf/config.json");

    rsst::initialize_logger(*config.get_should_log_debug());

    let db = Database::new(config.get_database_file(), SHOULD_DROP_TABLES_FIRST);

    let trigger_list: &Vec<Box<dyn Trigger>> = config.get_trigger_list();

    for feed_config in config.get_feed_config_list() {
        debug!("URL: {:?}", feed_config);
        continue;

        let xml: String = rsst::retrieve_xml(feed_config.get_url());

        match FeedType::new(&xml) {
            FeedType::Rss => {
                rsst::handle_rss_feed_case(&db, &xml, &trigger_list);
            }
            FeedType::Atom => {
                rsst::handle_atom_feed_case(&db, &xml, &trigger_list);
            }
            _ => {
                panic!("Unknown feed type.");
            }
        }
    }
}
