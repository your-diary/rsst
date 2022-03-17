use std::fs;

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

    let db = Database::new(DATABASE_FILE);

    //     let contents: String = fs::read_to_string("../samples/arch_linux.xml").unwrap();
    //     let contents: String = fs::read_to_string("../samples/sample-rss-2.xml").unwrap();
    let contents: String = fs::read_to_string("../samples/alpine_linux.xml").unwrap();
    //         let contents: String = String::from("hello");

    let discord = DiscordNotification::new(DISCORD_WEBHOOK_URL);

    let trigger_list: Vec<Box<dyn Trigger>> = vec![Box::new(discord)];

    match FeedType::new(&contents) {
        FeedType::Rss => {
            rsst::handle_rss_feed_case(&db, &contents, &trigger_list);
        }
        FeedType::Atom => {
            rsst::handle_atom_feed_case(&db, &contents, &trigger_list);
        }
        _ => {
            panic!("Unknown feed type.");
        }
    }
}
