use sea_orm::entity::prelude::*;

// Using the derive macro
#[derive(Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(Some(1))",
    enum_name = "category"
)]
pub enum AttachmentType {
    #[sea_orm(string_value = "Pdf")]
    Pdf,
    #[sea_orm(string_value = "Image")]
    Image,
    #[sea_orm(string_value = "Excel")]
    Excel,
    #[sea_orm(string_value = "Csv")]
    Csv,
    #[sea_orm(string_value = "Zip")]
    Zip,
}
