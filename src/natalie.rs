use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serenity::{json::Value, model::prelude::Embed, utils::Color};

use crate::selector::{self, UpdateFromData};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Article {
    pub url: String,
    pub image_url: String,
    pub title: String,
    pub summary: String,
    pub date: String,
}

impl Article {
    pub fn embed(&self) -> Value {
        Embed::fake(|e| {
            e.url(&self.url)
                .title(&self.title)
                .description(&self.summary)
                .thumbnail(&self.image_url)
                .color(Color::from_rgb(88, 255, 93))
        })
    }
}

impl UpdateFromData for Article {
    fn update_from_data(&mut self, selector: Option<&str>, r#type: &selector::Data, value: String) {
        match (selector, r#type) {
            (None, selector::Data::Attribute(attr)) if attr == "href" => self.url = value,
            (Some("img"), selector::Data::Attribute(attr)) if attr == "data-src" => {
                self.image_url = value;
            }
            (Some(".NA_card_text .NA_card_title"), selector::Data::Text) => self.title = value,
            (Some(".NA_card_text .NA_card_summary"), selector::Data::Text) => self.summary = value,
            (Some(".NA_card_text .NA_card_date"), selector::Data::Text) => self.date = value,
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

pub static NATALIE_SELECTOR: Lazy<selector::Element> = Lazy::new(|| selector::Element {
    selector: "main".into(),
    data: None,
    children: Some(vec![selector::Element {
        selector: ".NA_section-list".into(),
        data: None,
        children: Some(vec![selector::Element {
            selector: ".NA_card_wrapper".into(),
            data: None,
            children: Some(vec![selector::Element {
                selector: ".NA_card-l > a".into(),
                data: Some(vec![selector::Data::Attribute("href".into())]),
                children: Some(vec![
                    selector::Element {
                        selector: "img".into(),
                        data: Some(vec![selector::Data::Attribute("data-src".into())]),
                        children: None,
                    },
                    selector::Element {
                        selector: ".NA_card_text .NA_card_title".into(),
                        data: Some(vec![selector::Data::Text]),
                        children: None,
                    },
                    selector::Element {
                        selector: ".NA_card_text .NA_card_summary".into(),
                        data: Some(vec![selector::Data::Text]),
                        children: None,
                    },
                    selector::Element {
                        selector: ".NA_card_text .NA_card_date".into(),
                        data: Some(vec![selector::Data::Text]),
                        children: None,
                    },
                ]),
            }]),
        }]),
    }]),
});
