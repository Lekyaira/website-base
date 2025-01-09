use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241231_000001_post"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define the table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .col(ColumnDef::new(Post::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                    )
                    .col(ColumnDef::new(Post::Title)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(Post::Body)
                        .text()
                        .not_null()
                    )
                    .col(ColumnDef::new(Post::Published)
                        .boolean()
                        .not_null()
                        .default(false)
                    )
                    .to_owned()
            )
            .await
    }

    // Define how to roll back.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Post {
    Table,
    Id,
    Title,
    Body,
    Published,
}
