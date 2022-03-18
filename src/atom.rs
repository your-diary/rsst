use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use quick_xml::events::Event;
use quick_xml::Reader;

#[derive(Debug)]
pub struct Atom {
    title: String,
    id: String,
    entry_list: Vec<AtomEntry>,
}

#[derive(Debug, Hash, Clone)]
pub struct AtomEntry {
    title: String,
    id: String,
    updated: String,
    summary: Option<String>,
    content: Option<String>,
}

enum TagType {
    Other,
    Feed,
    FeedTitle,
    FeedId,
    Entry,
    EntryTitle,
    EntryId,
    EntryUpdated,
    EntrySummary,
    EntryContent,
}

impl Atom {
    pub fn new(xml: &str) -> Self {
        let mut ret = Atom {
            title: String::new(),
            id: String::new(),
            entry_list: Vec::new(),
        };

        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);
        reader.expand_empty_elements(true);

        let mut buf: Vec<u8> = Vec::new();

        let mut tag_stack: Vec<TagType> = Vec::new();

        loop {
            buf.clear();

            match reader.read_event(&mut buf) {
                Ok(Event::Eof) => {
                    break;
                }

                Ok(Event::Start(ref e)) => match e.name() {
                    b"feed" => {
                        tag_stack.push(TagType::Feed);
                    }
                    b"entry" => {
                        tag_stack.push(TagType::Entry);
                        ret.entry_list.push(AtomEntry::new());
                    }
                    b"title" => match tag_stack.last().unwrap() {
                        TagType::Feed => {
                            tag_stack.push(TagType::FeedTitle);
                        }
                        TagType::Entry => {
                            tag_stack.push(TagType::EntryTitle);
                        }
                        _ => (),
                    },
                    b"id" => match tag_stack.last().unwrap() {
                        TagType::Feed => {
                            tag_stack.push(TagType::FeedId);
                        }
                        TagType::Entry => {
                            tag_stack.push(TagType::EntryId);
                        }
                        _ => (),
                    },
                    b"updated" => match tag_stack.last().unwrap() {
                        TagType::Entry => {
                            tag_stack.push(TagType::EntryUpdated);
                        }
                        _ => (),
                    },
                    b"summary" => match tag_stack.last().unwrap() {
                        TagType::Entry => {
                            tag_stack.push(TagType::EntrySummary);
                        }
                        _ => (),
                    },
                    b"content" => match tag_stack.last().unwrap() {
                        TagType::Entry => {
                            tag_stack.push(TagType::EntryContent);
                        }
                        _ => (),
                    },
                    _ => {
                        tag_stack.push(TagType::Other);
                    }
                },

                Ok(Event::End(_)) => {
                    tag_stack.pop();
                }

                Ok(Event::Text(ref e)) => {
                    let text: String = e.unescape_and_decode(&reader).unwrap();
                    match tag_stack.last().unwrap() {
                        TagType::FeedTitle => {
                            ret.title = text;
                        }
                        TagType::FeedId => {
                            ret.id = text;
                        }
                        TagType::EntryTitle => {
                            ret.entry_list.last_mut().unwrap().title = text;
                        }
                        TagType::EntryId => {
                            ret.entry_list.last_mut().unwrap().id = text;
                        }
                        TagType::EntryUpdated => {
                            ret.entry_list.last_mut().unwrap().updated = text;
                        }
                        TagType::EntrySummary => {
                            ret.entry_list.last_mut().unwrap().summary = Some(text);
                        }
                        TagType::EntryContent => {
                            ret.entry_list.last_mut().unwrap().content = Some(text);
                        }
                        _ => (),
                    }
                }

                Err(e) => {
                    panic!("Error at position {}: {:?}", reader.buffer_position(), e)
                }

                _ => (),
            }
        }

        ret.entry_list.reverse();

        ret
    }

    pub fn hash_code(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.title.hash(&mut hasher);
        self.id.hash(&mut hasher);
        hasher.finish().to_string()
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_entry_list(&self) -> &Vec<AtomEntry> {
        &self.entry_list
    }
}

impl AtomEntry {
    fn new() -> Self {
        AtomEntry {
            id: String::new(),
            title: String::new(),
            updated: String::new(),
            summary: None,
            content: None,
        }
    }

    pub fn hash_code(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish().to_string()
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_updated(&self) -> &str {
        &self.updated
    }

    pub fn get_summary_or_content(&self) -> &Option<String> {
        match self.summary {
            Some(_) => &self.summary,
            None => &self.content,
        }
    }
}
