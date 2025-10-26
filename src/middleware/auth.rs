use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

use crate::{
    error::AppError,
    state::AppState,
    utils::jwt::{Claims, verify_token},
};

// Middleware that checks if user is authenticated
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Extract Authorization header
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized(
            "Missing authorization header".to_string(),
        ))?;

    // Check if it starts with "Bearer "
    let token = auth_header.strip_prefix("Bearer ").ok_or(
        AppError::Unauthorized("Invalid authorization format".to_string()),
    )?;

    // Verify token
    let claims = verify_token(token, &state.config.jwt_secret)?;

    // Add claims to request extensions (so handlers can access it)
    request.extensions_mut().insert(claims);

    // Continue to next handler
    Ok(next.run(request).await)
}

// Optional: Middleware to check for admin role
pub async fn admin_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Get claims from extensions (added by auth_middleware)
    let claims = request
        .extensions()
        .get::<Claims>()
        .ok_or(AppError::Unauthorized("Unauthorized".to_string()))?;

    // Check if user is admin
    if claims.role != "admin" {
        return Err(AppError::Unauthorized(
            "Admin access required".to_string(),
        ));
    }

    Ok(next.run(request).await)
}
