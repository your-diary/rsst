use std::rc::Rc;

use rusqlite::{params, types::Value, Connection};

use super::atom::Atom;
use super::atom::AtomEntry;
use super::rss::Rss;
use super::rss::RssItem;

// #[derive(Debug)]
// pub struct FeedsEntity {
//     hash: String,
//     title: String,
//     link: String,
// }

// pub struct FeedItemsEntity {
//     hash: String,
//     parent_hash: String,
//     title: Option<String>,
//     link: Option<String>,
//     description: Option<String>,
//     pub_date: Option<String>,
// }

// impl FeedsEntity {
//     pub fn new(hash: String, title: String, link: String) -> Self {
//         FeedsEntity { hash, title, link }
//     }
//     pub fn get_hash(&self) -> &str {
//         &self.hash
//     }
//     pub fn get_title(&self) -> &str {
//         &self.title
//     }
//     pub fn get_link(&self) -> &str {
//         &self.link
//     }
// }

pub struct Database {
    db_connection: Connection,
}

//misc
impl Database {
    pub fn new(database_file: &str) -> Self {
        let db_connection = Connection::open(database_file).unwrap();
        rusqlite::vtab::array::load_module(&db_connection).unwrap();

        //TODO: remove
        //         db_connection
        //             .execute(r#"DROP TABLE IF EXISTS "feed_items";"#, [])
        //             .unwrap();
        //         db_connection
        //             .execute(r#"DROP TABLE IF EXISTS "feeds";"#, [])
        //             .unwrap();

        //Represents each feed (each site).
        db_connection
            .execute(
                r#"
                    CREATE TABLE IF NOT EXISTS "feeds" (
                        "hash"  TEXT PRIMARY KEY,
                        "title" TEXT NOT NULL,
                        "link"  TEXT NOT NULL
                    )
                "#,
                [],
            )
            .unwrap();

        //Represents each feed item (each blog entry).
        db_connection
            .execute(
                r#"
                CREATE TABLE IF NOT EXISTS "feed_items" (
                    "hash"        TEXT PRIMARY KEY,
                    "parent_hash" TEXT,
                    "title"       TEXT,
                    "link"        TEXT,
                    "description" TEXT,
                    "pub_date"    TEXT,
                    FOREIGN KEY("parent_hash") REFERENCES feeds("hash")
                )
                "#,
                [],
            )
            .unwrap();

        Database { db_connection }
    }
}

//rss
impl Database {
    pub fn does_rss_feed_exist(&self, rss: &Rss) -> bool {
        let mut stmt = self
            .db_connection
            .prepare(r#"SELECT * FROM "feeds" WHERE "hash" = ?"#)
            .unwrap();
        stmt.exists([rss.hash_code()]).unwrap()
    }

    pub fn insert_rss_feed(&self, rss: &Rss) -> () {
        self.db_connection
            .execute(
                r#"INSERT INTO "feeds" ("hash", "title", "link") VALUES (?, ?, ?)"#,
                params![rss.hash_code(), rss.get_title(), rss.get_link()],
            )
            .unwrap();
    }

    pub fn select_rss_feed_items(&self, hash_list: &Vec<String>) -> Vec<String> {
        self.db_connection
            .prepare(r#"SELECT "hash" FROM "feed_items" WHERE "hash" IN rarray(?)"#)
            .unwrap()
            .query_map(
                params![Rc::new(
                    hash_list
                        .iter()
                        .cloned()
                        .map(Value::from)
                        .collect::<Vec<Value>>()
                )],
                |r| r.get::<_, String>(0),
            )
            .unwrap()
            .map(|e| e.unwrap())
            .collect()
    }

    pub fn insert_rss_feed_items(&self, parent_hash: &str, rss_items: &Vec<RssItem>) -> () {
        for rss_item in rss_items {
            self.db_connection
                .execute(
                    r#"INSERT INTO "feed_items" ("hash", "parent_hash", "title", "link", "description", "pub_date") VALUES (?, ?, ?, ?, ?, ?)"#,
                    params![rss_item.hash_code(), parent_hash, rss_item.get_title(), rss_item.get_link(), rss_item.get_description(), rss_item.get_pub_date()]
                )
                .unwrap();
        }
    }
}

//atom
impl Database {
    pub fn does_atom_feed_exist(&self, atom: &Atom) -> bool {
        let mut stmt = self
            .db_connection
            .prepare(r#"SELECT * FROM "feeds" WHERE "hash" = ?"#)
            .unwrap();
        stmt.exists([atom.hash_code()]).unwrap()
    }

    pub fn insert_atom_feed(&self, atom: &Atom) -> () {
        self.db_connection
            .execute(
                r#"INSERT INTO "feeds" ("hash", "title", "link") VALUES (?, ?, ?)"#,
                params![atom.hash_code(), atom.get_title(), atom.get_id()],
            )
            .unwrap();
    }

    pub fn select_atom_feed_entries(&self, hash_list: &Vec<String>) -> Vec<String> {
        self.db_connection
            .prepare(r#"SELECT "hash" FROM "feed_items" WHERE "hash" IN rarray(?)"#)
            .unwrap()
            .query_map(
                params![Rc::new(
                    hash_list
                        .iter()
                        .cloned()
                        .map(Value::from)
                        .collect::<Vec<Value>>()
                )],
                |r| r.get::<_, String>(0),
            )
            .unwrap()
            .map(|e| e.unwrap())
            .collect()
    }

    pub fn insert_atom_feed_entries(&self, parent_hash: &str, atom_entries: &Vec<AtomEntry>) -> () {
        for atom_entry in atom_entries {
            self.db_connection
                .execute(
                    r#"INSERT INTO "feed_items" ("hash", "parent_hash", "title", "link", "description", "pub_date") VALUES (?, ?, ?, ?, ?, ?)"#,
                    params![atom_entry.hash_code(), parent_hash, atom_entry.get_title(), atom_entry.get_id(), atom_entry.get_summary_or_content(), atom_entry.get_updated()]
                )
                .unwrap();
        }
    }
}
