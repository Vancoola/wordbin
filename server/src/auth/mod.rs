use crate::model::auth::{Claims, Role};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use time::{OffsetDateTime};
use uuid::Uuid;



pub fn create_token(secret: &str, role: Role, exp: Option<&OffsetDateTime>) -> anyhow::Result<String> {
    let claims = Claims {
        jti: Uuid::new_v4().to_string(),
        role,
        exp: exp.map(|t| t.unix_timestamp()),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

pub fn verify_token(token: &str, secret: &str) -> anyhow::Result<Claims> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.required_spec_claims.remove("exp");

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;
    Ok(data.claims)
}
