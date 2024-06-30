pub mod case_database;
pub mod case_handler;
pub mod collaboration_handler;
pub mod user_service;
pub mod token_service;


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ErrorResponse {
    pub error: String
}
