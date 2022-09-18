use anyhow::Result;
use sea_orm::{Database, DatabaseConnection};
use serenity::{http::Http, model::webhook::Webhook};
use std::env;

#[derive(Debug)]
pub struct Context {
    pub db: DatabaseConnection,
    pub webhook: Webhook,
    pub serenity_http: Http,
}

impl Context {
    pub async fn new_from_env() -> Result<Self> {
        let natalie_webhook_url = env::var("NATALIE_WEBHOOK_URL")?;

        let serenity_http = serenity::http::Http::new("");
        let webhook = Webhook::from_url(&serenity_http, &natalie_webhook_url).await?;

        let db_url = env::var("DATABASE_URL")?;
        let db = Database::connect(db_url).await?;

        Ok(Self {
            db,
            webhook,
            serenity_http,
        })
    }
}
