pub mod case_database;
pub mod core_database;
pub mod case_handler;
pub mod collaboration_handler;


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ErrorResponse {
    pub error: String
}