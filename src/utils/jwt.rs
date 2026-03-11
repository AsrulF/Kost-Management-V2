use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub role: String,
    pub permissions: Vec<String>,
    pub exp: usize,
}

// Helper function to generate jwt token
#[cfg(feature = "ssr")]
pub fn generate_token(user_id: Uuid, user_role: String, permissions: Vec<String>) -> Result<String, jsonwebtoken::errors::Error> {
    use jsonwebtoken::{
        encode, Header, EncodingKey,
    };
    use chrono::{Utc, Duration};
    
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
        &EncodingKey::from_secret(
            std::env::var("JWT_SECRET_KEY")
                .unwrap_or_else(|_| "kost-management-v2".to_string())
                .as_ref()
        ) 
        
    )
}


#[cfg(feature = "ssr")]
pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    use jsonwebtoken::{
        decode, Header, DecodingKey,
        Validation,
    };
    
    // Decode the token and verify
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(
            std::env::var("JWT_SECRET_KEY")
                .unwrap_or_else(|_| "kost-management-v2".to_string())
                .as_ref()   
        ),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}