use sea_orm_migration::{
    prelude::*,
    sea_orm::{EnumIter, Iterable},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        let _ = manager
            .create_table(
                Table::create()
                    .table(Todo::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Todo::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Todo::Title).string().not_null())
                    .col(ColumnDef::new(Todo::Text).string().not_null())
                    .to_owned(),
            )
            .await;

        manager
            .create_table(
                Table::create()
                    .table(Attachment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Attachment::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Attachment::Todo)
                            .integer()
                            .default(Value::Int(None)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("attachment_type")
                            .from(Attachment::Table, Attachment::Todo)
                            .to(Todo::Table, Todo::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Todo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Todo {
    Table,
    Id,
    Title,
    Text,
}

#[derive(Iden, EnumIter)]
enum Attachment {
    Table,
    Id,
    Todo,
    Image,
    Pdf,
    File,
}

#[derive(Iden, EnumIter)]
enum Pdf {
    Table,
    Id,
    Title,
    Orientation,
}

#[derive(Iden, EnumIter)]
enum Image {
    Table,
    Id,
    Title,
    Resolution,
}
