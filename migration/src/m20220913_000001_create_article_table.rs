use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220913_000001_create_article_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Article::Table)
                    .col(
                        ColumnDef::new(Article::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Article::Uid).uuid().not_null().unique_key())
                    .col(ColumnDef::new(Article::Url).string().not_null())
                    .col(
                        ColumnDef::new(Article::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    // .index(
                    // ,
                    // )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Article::Table)
                    .name("idx-created-at-desc")
                    .col((Article::CreatedAt, IndexOrder::Desc))
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Article::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Article {
    Table,
    Id,
    Uid,
    Url,
    CreatedAt,
}
