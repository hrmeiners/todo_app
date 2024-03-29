use sea_orm_migration::prelude::*;


pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240307_162300_create_tasks_table.rs"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(Tasks::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tasks::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Tasks::Item).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                .table(Tasks::Table)
                .to_owned()
            )
            .await
    }
}

#[derive(Iden)]
enum Tasks {
    Table,
    Id,
    Item
}
