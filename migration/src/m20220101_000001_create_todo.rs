use sea_orm_migration::{
    prelude::*,
    sea_orm::{DeriveActiveEnum, EnumIter},
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
                    .col(ColumnDef::new(Todo::DueDate).date().not_null())
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
                    .col(ColumnDef::new(Attachment::Name).string().not_null())
                    .col(ColumnDef::new(Attachment::Url).string().not_null())
                    .col(ColumnDef::new(Attachment::AttachmentType).enumeration(
                        AttachmentType::Table,
                        [
                            AttachmentType::Csv,
                            AttachmentType::Pdf,
                            AttachmentType::Image,
                            AttachmentType::Excel,
                            AttachmentType::Zip,
                        ],
                    ))
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
    DueDate,
}

#[derive(Iden, EnumIter)]
enum Attachment {
    Table,
    Id,
    Todo,
    Name,
    Url,
    AttachmentType,
}

#[derive(Debug, PartialEq, Eq, Iden, EnumIter, DeriveActiveEnum)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(Some(1))",
    enum_name = "TypeAttribute"
)]
enum TypeAttribute {
    #[sea_orm(string_value = "TypeAttribute")]
    Table,
    #[sea_orm(string_value = "Orientation")]
    Orientation,
    #[sea_orm(string_value = "Quality")]
    Quality,
    #[sea_orm(string_value = "Encoding")]
    Encoding,
    #[sea_orm(string_value = "Local")]
    Local,
    #[sea_orm(string_value = "Delimiter")]
    Delimiter,
    #[sea_orm(string_value = "QuoteChar")]
    QuoteChar,
    #[sea_orm(string_value = "Filename")]
    Filename,
}

#[derive(Debug, PartialEq, Eq, Iden, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum AttachmentType {
    #[sea_orm(string_value = "AttachmentType")]
    Table,
    #[sea_orm(string_value = "Pdf")]
    #[iden = "Pdf"]
    Pdf,
    #[sea_orm(string_value = "Image")]
    #[iden = "Image"]
    Image,
    #[sea_orm(string_value = "Excel")]
    #[iden = "Excel"]
    Excel,
    #[sea_orm(string_value = "Csv")]
    #[iden = "Csv"]
    Csv,
    #[sea_orm(string_value = "Zip")]
    #[iden = "Zip"]
    Zip,
    #[sea_orm(string_value = "AttachmentAttribute")]
    AttachmentAttribute,
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
