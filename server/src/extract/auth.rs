use crate::auth::verify_token;
use crate::state::AppState;
use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use crate::model::auth::Role;

pub struct Authenticated {
    pub role: Role,
}

impl FromRequestParts<AppState> for Authenticated {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header"))?;

        let claims = verify_token(token, state.app_config.security.jwt.secret.as_ref())
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token"))?;
        Ok(Authenticated { role: claims.role })
    }
}
