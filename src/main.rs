#![warn(clippy::pedantic)]

use std::collections::HashSet;

use anyhow::Result;
use hawkeye_entity as entity;
use hawkeye_entity::prelude::Article as ArticleEntity;
use sea_orm::{ActiveValue::NotSet, ColumnTrait, EntityTrait, QueryFilter, QuerySelect, Set};
use tokio::time::{self, Duration};

mod selector;
use selector::fetch_and_select;

mod natalie;
use natalie::{Article, NATALIE_SELECTOR, NATALIE_URLS};

mod webhook;

mod util;
use util::Context;

#[tokio::main]
async fn main() -> Result<()> {
    let ctx = Context::new_from_env().await?;

    let mut interval = time::interval(Duration::from_secs(60));
    let mut i = 0;
    loop {
        interval.tick().await;

        if let Err(err) = fetch_and_notify(&ctx, i).await {
            eprintln!("{err:?}");
        }

        i = (i + 1) % NATALIE_URLS.len();
    }
}

async fn fetch_and_notify(ctx: &Context, i: usize) -> Result<()> {
    let articles = fetch_and_select::<Article>(NATALIE_URLS[i], &NATALIE_SELECTOR).await?;
    let urls = articles.iter().map(|a| &*a.url);
    let posted_articles: HashSet<_> = ArticleEntity::find()
        .filter(entity::article::Column::Url.is_in(urls))
        .limit(articles.len() as u64)
        .all(&ctx.db)
        .await?
        .iter()
        .map(|article| article.url.clone())
        .collect();

    let new_articles = articles
        .iter()
        .rev()
        .filter(|article| !posted_articles.contains(&article.url))
        .collect::<Vec<_>>();

    let article_entities = new_articles
        .iter()
        .map(|article| entity::article::ActiveModel {
            id: NotSet,
            uid: Set(uuid::Uuid::new_v4()),
            url: Set(article.url.clone()),
            created_at: Set(chrono::offset::Utc::now().into()),
        })
        .collect::<Vec<_>>();

    if !article_entities.is_empty() {
        ArticleEntity::insert_many(article_entities)
            .exec(&ctx.db)
            .await?;
    }

    for article in &new_articles {
        webhook::send(ctx, article.embed()).await?;
    }

    Ok(())
}
