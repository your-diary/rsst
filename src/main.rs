use std::fs;

use rsst;
use rsst::atom::Atom;
use rsst::feedtype::FeedType;
use rsst::rss::Rss;

fn main() {
    let contents: String = fs::read_to_string("../samples/arch_linux.xml").unwrap();
    //     let contents: String = fs::read_to_string("../samples/sample-rss-2.xml").unwrap();
    //     let contents: String = fs::read_to_string("../samples/alpine_linux.xml").unwrap();
    //     let contents: String = String::from("hello");

    match FeedType::new(&contents) {
        FeedType::Rss => {
            let rss = Rss::new(&contents);
            println!("{:?}", rss);
            rss.get_item_list()
                .iter()
                .for_each(|e| println!("{}", e.hash_code()));
        }
        FeedType::Atom => {
            let atom = Atom::new(&contents);
            println!("{:?}", atom);
            atom.get_entry_list()
                .iter()
                .for_each(|e| println!("{}", e.hash_code()));
        }
        _ => {
            panic!("unknown");
        }
    }
}
