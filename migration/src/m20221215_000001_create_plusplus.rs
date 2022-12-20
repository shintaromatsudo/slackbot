use entity::plusplus;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20221215_000001_create_plusplus"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(plusplus::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(plusplus::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(plusplus::Column::SlackId)
                            .unique_key()
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(plusplus::Column::Counter)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(plusplus::Column::Deleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(plusplus::Column::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(plusplus::Column::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(plusplus::Entity).to_owned())
            .await
    }
}
