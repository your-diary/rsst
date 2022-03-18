use std::rc::Rc;

use rusqlite::{params, types::Value, Connection};

use super::atom::Atom;
use super::atom::AtomEntry;
use super::rss::Rss;
use super::rss::RssItem;

pub struct Database {
    db_connection: Connection,
}

//misc
impl Database {
    pub fn new(database_file: &str, should_drop_tables_first: bool) -> Self {
        let db_connection = Connection::open(database_file).unwrap();
        rusqlite::vtab::array::load_module(&db_connection).unwrap();

        //for debug
        if (should_drop_tables_first) {
            db_connection
                .execute(r#"DROP TABLE IF EXISTS "feed_items";"#, [])
                .unwrap();
            db_connection
                .execute(r#"DROP TABLE IF EXISTS "feeds";"#, [])
                .unwrap();
        }

        Database::initialize_database(&db_connection);

        Database { db_connection }
    }

    fn initialize_database(db_connection: &Connection) {
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
    }

    pub fn does_feed_exist(&self, hash_code: &str) -> bool {
        let mut stmt = self
            .db_connection
            .prepare(r#"SELECT * FROM "feeds" WHERE "hash" = ?"#)
            .unwrap();
        stmt.exists([hash_code]).unwrap()
    }

    fn insert_into_feeds(&self, hash_code: &str, title: &str, link: &str) -> () {
        self.db_connection
            .execute(
                r#"INSERT INTO "feeds" ("hash", "title", "link") VALUES (?, ?, ?)"#,
                params![hash_code, title, link],
            )
            .unwrap();
    }

    pub fn select_feed_items(&self, hash_list: &Vec<String>) -> Vec<String> {
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
}

//rss
impl Database {
    pub fn insert_rss_feed(&self, rss: &Rss) -> () {
        self.insert_into_feeds(&rss.hash_code(), rss.get_title(), rss.get_link());
    }

    pub fn insert_rss_feed_items(&self, parent_hash: &str, rss_items: &Vec<RssItem>) -> () {
        for rss_item in rss_items {
            self.db_connection
                .execute(
                    r#"
                        INSERT INTO "feed_items"
                        ("hash", "parent_hash", "title", "link", "description", "pub_date")
                        VALUES (?, ?, ?, ?, ?, ?)
                    "#,
                    params![
                        rss_item.hash_code(),
                        parent_hash,
                        rss_item.get_title(),
                        rss_item.get_link(),
                        rss_item.get_description(),
                        rss_item.get_pub_date()
                    ],
                )
                .unwrap();
        }
    }
}

//atom
impl Database {
    pub fn insert_atom_feed(&self, atom: &Atom) -> () {
        self.insert_into_feeds(&atom.hash_code(), atom.get_title(), atom.get_id());
    }

    pub fn insert_atom_feed_entries(&self, parent_hash: &str, atom_entries: &Vec<AtomEntry>) -> () {
        for atom_entry in atom_entries {
            self.db_connection
                .execute(
                    r#"
                        INSERT INTO "feed_items"
                        ("hash", "parent_hash", "title", "link", "description", "pub_date")
                        VALUES (?, ?, ?, ?, ?, ?)
                    "#,
                    params![
                        atom_entry.hash_code(),
                        parent_hash,
                        atom_entry.get_title(),
                        atom_entry.get_id(),
                        atom_entry.get_summary_or_content(),
                        atom_entry.get_updated()
                    ],
                )
                .unwrap();
        }
    }
}
