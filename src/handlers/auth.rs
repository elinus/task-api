use axum::{Json, extract::State};
use bcrypt::{DEFAULT_COST, hash, verify};
use sqlx::PgPool;
use validator::Validate;

use crate::{
    config::Config,
    error::Result,
    models::{AuthResponse, LoginRequest, RegisterRequest, User},
    state::AppState,
    utils::jwt::{Claims, create_token},
};

// Register new user
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>> {
    // Validate input
    payload.validate()?;

    // Hash password
    let password_hash = hash(&payload.password, DEFAULT_COST).map_err(|e| {
        crate::error::AppError::InternalError(format!("Hashing failed: {}", e))
    })?;

    // Create user in database
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (email, password_hash, role)
        VALUES ($1, $2, 'user')
        RETURNING *
        "#,
    )
    .bind(&payload.email)
    .bind(&password_hash)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        // Check if email already exists
        if e.to_string().contains("unique") {
            crate::error::AppError::ValidationError(
                validator::ValidationErrors::new(),
            )
        } else {
            e.into()
        }
    })?;

    // Create JWT token
    let claims = Claims::new(
        user.id,
        user.email.clone(),
        user.role.clone(),
        state.config.jwt_expiration_hours,
    );

    let token = create_token(&claims, &state.config.jwt_secret)?;

    Ok(Json(AuthResponse {
        token,
        user_id: user.id.to_string(),
        email: user.email,
    }))
}

// Login existing user
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>> {
    // Validate input
    payload.validate()?;

    // Find user by email
    let user =
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(&payload.email)
            .fetch_optional(&state.pool)
            .await?
            .ok_or(crate::error::AppError::Unauthorized(
                "Invalid email or password".to_string(),
            ))?;

    // Verify password
    let password_valid = verify(&payload.password, &user.password_hash)
        .map_err(|e| {
            crate::error::AppError::InternalError(format!(
                "Verification failed: {}",
                e
            ))
        })?;

    if !password_valid {
        return Err(crate::error::AppError::Unauthorized(
            "Invalid email or password".to_string(),
        ));
    }

    // Create JWT token
    let claims = Claims::new(
        user.id,
        user.email.clone(),
        user.role.clone(),
        state.config.jwt_expiration_hours,
    );

    let token = create_token(&claims, &state.config.jwt_secret)?;

    Ok(Json(AuthResponse {
        token,
        user_id: user.id.to_string(),
        email: user.email,
    }))
}
