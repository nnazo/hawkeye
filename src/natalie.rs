use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::select::{DataSelector, ElementSelector, UpdateFromSelected};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub url: String,
    pub image_url: String,
    pub title: String,
    pub summary: String,
    pub date: String,
}

impl UpdateFromSelected for Article {
    fn new() -> Self {
        Article {
            url: "".into(),
            image_url: "".into(),
            title: "".into(),
            summary: "".into(),
            date: "".into(),
        }
    }

    fn update(&mut self, selector: Option<&str>, r#type: &DataSelector, value: String) {
        match (selector, r#type) {
            (None, DataSelector::Attribute(attr)) if attr == "href" => self.url = value,
            (Some("img"), DataSelector::Attribute(attr)) if attr == "data-src" => {
                self.image_url = value
            }
            (Some(".NA_card_text .NA_card_title"), DataSelector::Text) => self.title = value,
            (Some(".NA_card_text .NA_card_summary"), DataSelector::Text) => self.summary = value,
            (Some(".NA_card_text .NA_card_date"), DataSelector::Text) => self.date = value,
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Articles(pub Vec<Article>);

pub const NATALIE_URLS: [&str; 3] = [
    "https://natalie.mu/comic/tag/43",
    "https://natalie.mu/comic/tag/42",
    "https://natalie.mu/comic/tag/59",
];

pub static NATALIE_SELECTOR: Lazy<ElementSelector> = Lazy::new(|| ElementSelector {
    selector: "main".into(),
    data: None,
    children: Some(vec![ElementSelector {
        selector: ".NA_section-list".into(),
        data: None,
        children: Some(vec![ElementSelector {
            selector: ".NA_card_wrapper".into(),
            data: None,
            children: Some(vec![ElementSelector {
                selector: ".NA_card-l > a".into(),
                data: Some(vec![DataSelector::Attribute("href".into())]),
                children: Some(vec![
                    ElementSelector {
                        selector: "img".into(),
                        data: Some(vec![DataSelector::Attribute("data-src".into())]),
                        children: None,
                    },
                    ElementSelector {
                        selector: ".NA_card_text .NA_card_title".into(),
                        data: Some(vec![DataSelector::Text]),
                        children: None,
                    },
                    ElementSelector {
                        selector: ".NA_card_text .NA_card_summary".into(),
                        data: Some(vec![DataSelector::Text]),
                        children: None,
                    },
                    ElementSelector {
                        selector: ".NA_card_text .NA_card_date".into(),
                        data: Some(vec![DataSelector::Text]),
                        children: None,
                    },
                ]),
            }]),
        }]),
    }]),
});
