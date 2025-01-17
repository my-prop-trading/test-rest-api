service_sdk::macros::use_my_postgres!();
use serde_derive::Serialize;

#[derive(SelectDbEntity, InsertDbEntity, UpdateDbEntity, TableSchema, Serialize, Clone)]
pub struct PersonalDataDto {
    #[primary_key(0)]
    pub id: String,

    pub first_name: Option<String>,

    pub last_name: Option<String>,

    #[db_index(id=1, index_name: "email_idx", is_unique: false, order: "ASC")]
    pub email: Option<String>,

    #[order_by_desc]
    #[db_index(id=2, index_name: "created_at_idx", is_unique: false, order: "DESC")]
    #[sql_type("timestamp")]
    pub created_at: DateTimeAsMicroseconds,
}

#[derive(WhereDbModel)]
pub struct WhereByIdModel<'s> {
    pub id: &'s str,
}
