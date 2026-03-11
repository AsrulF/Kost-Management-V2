use jsonwebtoken::{
    encode, decode, Header, EncodingKey, DecodingKey,
    Validation, errors::Error as JwtError
};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub role: String,
    pub permissions: Vec<String>,
    pub exp: usize,
}

// Helper function to generate jwt token
pub fn generate_token(user_id: Uuid, user_role: String, permissions: Vec<String>) -> Result<String, JwtError> {
    // Set expiration token
    let exp = Utc::now()
        .checked_add_signed(Duration::minutes(10))
        .unwrap()
        .timestamp() as usize;

    // Make claims 
    encode(
        &Header::default(), 
        &Claims {
            sub: user_id,
            role: user_role,
            permissions,
            exp,
        }, 
        
    )
}