use serde::{Serialize, Deserialize};




#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,         // Optional. Audience
    pub exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iss: String,         // Optional. Issuer
    pub sub: String,         // Optional. Subject (whom token refers to)
}



