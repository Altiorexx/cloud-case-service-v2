use serde::{Deserialize, Serialize};



#[derive(Debug, Deserialize, Serialize)]
pub struct CheckTokenBody {
    pub token: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CheckTokenErrorResponse {
    pub error: String
}



