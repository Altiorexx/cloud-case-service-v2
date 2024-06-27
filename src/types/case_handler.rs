use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCIS18CaseBody {
    pub user_id: String,
    pub group_id: String,
    pub name: String,
    pub framework: String,
    pub implementation_group: i32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCaseResponse {
    pub case_id: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RenameCaseBody {
    pub name: String
}