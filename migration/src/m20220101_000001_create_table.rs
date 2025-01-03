use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Schedule {
    #[sea_orm(iden = "schedule")]
    Table,
    Id,
    Content,
    Date,
    Remind,
    Repeat,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Schedule::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Schedule::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Schedule::Content).text())
                    .col(ColumnDef::new(Schedule::Date).timestamp().not_null())
                    .col(ColumnDef::new(Schedule::Remind).boolean().default(false))
                    .col(ColumnDef::new(Schedule::Repeat).text().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Schedule::Table).to_owned())
            .await
    }
}
