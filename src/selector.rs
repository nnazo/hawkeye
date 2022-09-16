use anyhow::{anyhow, Result};
use fantoccini::ClientBuilder;
use scraper::{html::Html, selector::Selector, ElementRef};

/// Selector for an element in an HTML tree
#[derive(Debug, Clone)]
pub struct Element {
    pub selector: String,
    pub children: Option<Vec<Element>>,
    pub data: Option<Vec<Data>>,
}

/// Selector for data within an HTML element
#[derive(Debug, Clone)]
pub enum Data {
    Attribute(String),
    Text,
}

impl Data {
    fn select_from_ref(&self, el_ref: &ElementRef, selector: Option<&str>) -> Result<String> {
        match selector {
            Some(selector) => {
                let s =
                    Selector::parse(selector).map_err(|_| anyhow!("could not parse selector"))?;
                let el = el_ref
                    .select(&s)
                    .next()
                    .ok_or_else(|| anyhow!("no element from '{selector:?}'"))?;
                self.select_from(&el, Some(selector))
            }
            _ => self.select_from(el_ref, None),
        }
    }

    fn select_from(&self, el: &ElementRef, selector: Option<&str>) -> Result<String> {
        Ok(match self {
            Data::Text => el
                .text()
                .next()
                .ok_or_else(|| anyhow!("no text in '{selector:?}'"))?
                .to_string(),
            Data::Attribute(attr) => el
                .value()
                .attr(attr)
                .ok_or_else(|| anyhow!("no '{attr}' attr"))?
                .to_string(),
        })
    }
}

pub trait UpdateFromData {
    /// Updates data from data selected from an HTML element
    fn update_from_data(&mut self, selector: Option<&str>, r#type: &Data, value: String);
}

pub async fn fetch_and_select<T>(url: &str, selector: &Element) -> Result<Vec<T>>
where
    T: UpdateFromData + Default,
{
    let mut caps = serde_json::map::Map::new();
    let chrome_opts = serde_json::json!({ "args": ["--headless", "--disable-gpu", "--disable-dev-shm-usage", "--no-sandbox"] });
    caps.insert("goog:chromeOptions".to_string(), chrome_opts);
    let webdriver_client = ClientBuilder::rustls()
        .capabilities(caps)
        .connect(&std::env::var("BROWSER_DRIVER_URL")?)
        .await?;

    webdriver_client.goto(url).await?;
    let html = webdriver_client.source().await?;
    let document = Html::parse_document(&html);

    select_from_document(&document, selector)
}

fn select_from_document<T>(document: &Html, selector: &Element) -> Result<Vec<T>>
where
    T: UpdateFromData + Default,
{
    let s = Selector::parse(&selector.selector).map_err(|_| anyhow!("could not parse selector"))?;
    let elements = document.select(&s);

    let children = match selector.children.as_ref() {
        Some(children) => children,
        _ => return Ok(vec![]),
    };

    Ok(elements
        .flat_map(|el| {
            children
                .iter()
                .flat_map(move |child_selector| select_from_element(el, child_selector))
        })
        .flatten()
        .collect())
}

fn select_from_element<T>(el: ElementRef, selector: &Element) -> Result<Vec<T>>
where
    T: UpdateFromData + Default,
{
    let s = Selector::parse(&selector.selector).map_err(|_| anyhow!("could not parse selector"))?;
    let elements = el.select(&s);

    let children = match selector.children.as_ref() {
        Some(children) if children.len() == 1 => children,
        Some(children)
            if children.len() > 1 || (children[0].children.is_none() && children.len() == 1) =>
        {
            return Ok(elements
                .map(|el| {
                    let default_types = &vec![];
                    let types = selector.data.as_ref().unwrap_or(default_types);
                    let root_values = types.iter().map(|t| {
                        t.select_from_ref(&el, None)
                            .map(|value| (Option::<&str>::None, t, value))
                    });

                    let mut ret = T::default();
                    let mut values = children
                        .iter()
                        .flat_map(|child| {
                            let types = child.data.as_ref().unwrap_or(default_types);

                            types.iter().map(|t| {
                                t.select_from_ref(&el, Some(&child.selector))
                                    .map(|value| (Some(&*child.selector), t, value))
                            })
                        })
                        .chain(root_values);

                    while let Some(Ok((selector, t, value))) = values.next() {
                        ret.update_from_data(selector, t, value);
                    }
                    ret
                })
                .collect());
        }
        _ => return Err(anyhow!("got here somehow")),
    };

    Ok(elements
        .flat_map(|el| {
            children
                .iter()
                .flat_map(move |child_selector| select_from_element(el, child_selector))
        })
        .flatten()
        .collect())
}
