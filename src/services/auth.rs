use crate::models::message::{
    FAILED_TO_GENERATE_TOKEN, FAILED_TO_VALIDATE_TOKEN, LOGIN_FAILED, SUCCESS_TO_VALIDATE_TOKEN,
};
use crate::{config::settings::Settings, models::auth::Claims};
use axum::Error;
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};

pub struct AuthService {
    pub settings: Settings,
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            settings: Settings::new(),
        }
    }

    fn generate_token(&self, username: String) -> Result<String, String> {
        let jwt_secret = &self.settings.jwt_secret;
        let header = Header::new(Algorithm::HS256);
        let claims = Claims {
            username: username,
            exp: (Utc::now() + Duration::hours(self.settings.jwt_exp)).timestamp(),
        };
        let token = encode(
            &header,
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_bytes()),
        );
        match token {
            Ok(token) => Ok(token),
            Err(_) => Err(FAILED_TO_GENERATE_TOKEN.to_string()),
        }
    }

    fn validate_token(&self, token: String) -> Result<String, String> {
        let jwt_secret = &self.settings.jwt_secret;
        let token = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        );
        match token {
            Ok(_) => Ok(SUCCESS_TO_VALIDATE_TOKEN.to_string()),
            Err(_) => Err(FAILED_TO_VALIDATE_TOKEN.to_string()),
        }
    }

    pub fn login(&self, username: String, password: String) -> Result<String, String> {
        if username != self.settings.username {
            return Err(LOGIN_FAILED.to_string());
        }
        if password != self.settings.password {
            return Err(LOGIN_FAILED.to_string());
        }
        match self.generate_token(username) {
            Ok(token) => Ok(token),
            Err(_) => Err(FAILED_TO_GENERATE_TOKEN.to_string()),
        }
    }

    pub fn verify_token(&self, token: String) -> Result<String, Error> {
        match self.validate_token(token) {
            Ok(_) => Ok(SUCCESS_TO_VALIDATE_TOKEN.to_string()),
            Err(_) => Err(Error::new(FAILED_TO_VALIDATE_TOKEN.to_string())),
        }
    }
}
