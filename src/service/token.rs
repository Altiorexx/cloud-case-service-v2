use std::env;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};

use crate::types::token_service::Claims;


pub struct TokenService {
    issuer: String,
    secret: String,
    domain: String
}

impl TokenService {

    pub fn new() -> Self {
        Self {
            issuer: env::var("ISSUER").expect("ISSUER environment variable"),
            secret: env::var("SECRET").expect("SECRET environment variable"),
            domain: env::var("DOMAIN").expect("DOMAIN environment variable")
        }
    }


    pub fn new_token(&self, aud: &String) -> Result<String, jsonwebtoken::errors::Error> {

        let exp = Utc::now()
            .checked_add_signed(Duration::minutes(5))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            aud: aud.to_string(),
            exp,
            iss: self.issuer.to_string(),
            sub: "ClientTokenVerification".into()
        };
        
        let mut header = Header::default();
        header.alg = Algorithm::HS256;

        encode(&Header::default(), &claims, &EncodingKey::from_secret(self.secret.as_ref()))
    }

    pub fn check_token(&self, token: &str) -> bool {
        // Define the validation parameters
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        validation.set_issuer(self.issuer.as_ref());

        match decode::<Claims>(token, &DecodingKey::from_secret(self.secret.as_ref()), &validation) {
            Ok(parsed) => {

                // check if the token was meant for this service
                if parsed.claims.aud != self.domain {
                    return false;
                }

                // check that the token was addressed from a known service
                let known_issuers = vec!["https://portal.altiore.io", "https://user.service.altiore.io", "http://localhost:3000"];
                if known_issuers.contains(&parsed.claims.iss.as_str()) {
                    return false;
                }
                true
            },
            Err(_) => false,
        }
    }

}