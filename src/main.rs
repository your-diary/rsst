use log::*;

use rsst;
use rsst::database::Database;
use rsst::discord::DiscordNotification;
use rsst::feedtype::FeedType;
use rsst::trigger::Trigger;

fn main() {
    const SHOULD_LOG_DEBUG: bool = true;
    rsst::initialize_logger(SHOULD_LOG_DEBUG);

    const DATABASE_FILE: &str = "test.sqlite3";
    const DISCORD_WEBHOOK_URL: &str = "https://discord.com/api/webhooks/915979592320294922/idTy3fQi4khopKjbSe0V4ZtxwDhcSWWvykWkK27Isi0lEJPHnAb0TR7Mx-G5HQQAg_ji";

    const SHOULD_DROP_TABLES_FIRST: bool = false;

    let db = Database::new(DATABASE_FILE, SHOULD_DROP_TABLES_FIRST);

    let discord = DiscordNotification::new(DISCORD_WEBHOOK_URL);

    let trigger_list: Vec<Box<dyn Trigger>> = vec![Box::new(discord)];

    let url_list = vec!["http://localhost:9009/"];

    for url in url_list {
        debug!("URL: {}", url);

        let xml: String = rsst::retrieve_xml(url);

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
