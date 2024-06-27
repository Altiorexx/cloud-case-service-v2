use sqlx::FromRow;
use serde::Serialize;


#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    #[sqlx(rename = "lastLogin")]
    pub last_login: String,
    pub verified: bool
}

#[derive(Debug, FromRow, Serialize)]
pub struct Service {
    pub id: String,
    pub name: String,
    #[sqlx(rename = "implementationGroup")]
    pub implementation_group: Option<i32>,
    pub description: String
}


