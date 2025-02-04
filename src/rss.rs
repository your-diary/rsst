use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;

use quick_xml::events::Event;
use quick_xml::Reader;

use super::feedconfig::FeedConfig;

#[derive(Debug)]
pub struct Rss {
    title: String,
    link: String,
    description: String,
    item_list: Vec<RssItem>,
    feed_config: Rc<FeedConfig>,
}

#[derive(Debug, Clone)]
pub struct RssItem {
    title: Option<String>,
    link: Option<String>,
    description: Option<String>,
    pub_date: Option<String>,
    feed_config: Rc<FeedConfig>,
}

#[derive(Debug)]
enum TagType {
    Other,
    Channel,
    ChannelTitle,
    ChannelLink,
    ChannelDescription,
    Item,
    ItemTitle,
    ItemLink,
    ItemDescription,
    ItemPubDate,
}

impl Rss {
    pub fn new(xml: &str, feed_config: &FeedConfig) -> Self {
        let mut ret = Rss {
            title: String::new(),
            link: String::new(),
            description: String::new(),
            item_list: Vec::new(),
            feed_config: Rc::new(feed_config.clone()),
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
                    b"channel" => {
                        tag_stack.push(TagType::Channel);
                    }
                    b"item" => {
                        tag_stack.push(TagType::Item);
                        ret.item_list.push(RssItem::new(&ret.feed_config));
                    }
                    b"title" => match tag_stack.last().unwrap() {
                        TagType::Channel => {
                            tag_stack.push(TagType::ChannelTitle);
                        }
                        TagType::Item => {
                            tag_stack.push(TagType::ItemTitle);
                        }
                        _ => tag_stack.push(TagType::Other),
                    },
                    b"link" => match tag_stack.last().unwrap() {
                        TagType::Channel => {
                            tag_stack.push(TagType::ChannelLink);
                        }
                        TagType::Item => {
                            tag_stack.push(TagType::ItemLink);
                        }
                        _ => tag_stack.push(TagType::Other),
                    },
                    b"description" => match tag_stack.last().unwrap() {
                        TagType::Channel => {
                            tag_stack.push(TagType::ChannelDescription);
                        }
                        TagType::Item => {
                            tag_stack.push(TagType::ItemDescription);
                        }
                        _ => tag_stack.push(TagType::Other),
                    },
                    b"pubDate" => {
                        if let TagType::Item = tag_stack.last().unwrap() {
                            tag_stack.push(TagType::ItemPubDate);
                        }
                    }
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
                        TagType::ChannelTitle => {
                            ret.title = text;
                        }
                        TagType::ChannelLink => {
                            ret.link = text;
                        }
                        TagType::ChannelDescription => {
                            ret.description = text;
                        }
                        TagType::ItemTitle => {
                            ret.item_list.last_mut().unwrap().title = Some(text);
                        }
                        TagType::ItemLink => {
                            ret.item_list.last_mut().unwrap().link = Some(text);
                        }
                        TagType::ItemDescription => {
                            ret.item_list.last_mut().unwrap().description = Some(text);
                        }
                        TagType::ItemPubDate => {
                            ret.item_list.last_mut().unwrap().pub_date = Some(text);
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

        ret
    }

    pub fn hash_code(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.title.hash(&mut hasher);
        self.link.hash(&mut hasher);
        self.description.hash(&mut hasher);
        hasher.finish().to_string()
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_link(&self) -> &str {
        &self.link
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_item_list(&self) -> &Vec<RssItem> {
        &self.item_list
    }
}

impl RssItem {
    fn new(feed_config: &Rc<FeedConfig>) -> Self {
        RssItem {
            title: None,
            link: None,
            description: None,
            pub_date: None,
            feed_config: Rc::clone(feed_config),
        }
    }

    //We intentionally omit `self.description` as some feed suppliers often (e.g. everyday) update its value.
    pub fn hash_code(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.title.hash(&mut hasher);
        self.link.hash(&mut hasher);
        if !self.feed_config.should_omit_date_field_from_hash {
            self.pub_date.hash(&mut hasher);
        }
        hasher.finish().to_string()
    }

    pub fn get_title(&self) -> &Option<String> {
        &self.title
    }

    pub fn get_link(&self) -> &Option<String> {
        &self.link
    }

    pub fn get_description(&self) -> &Option<String> {
        &self.description
    }

    pub fn get_pub_date(&self) -> &Option<String> {
        &self.pub_date
    }
}
