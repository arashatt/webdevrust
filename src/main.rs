use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

// ============================================================================
// Configuration
// ============================================================================

const JWT_SECRET: &[u8] = b"your-secret-key-change-this-in-production";
const JWT_EXPIRATION_HOURS: i64 = 24;

// ============================================================================
// Types & Data Structures
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // subject (user ID)
    pub exp: i64,    // expiration time
    pub iat: i64,    // issued at
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    #[serde(skip)]
    pub password_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

// Application state
#[derive(Clone)]
pub struct AppState {
    users: Arc<RwLock<Vec<User>>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

// ============================================================================
// JWT Token Management
// ============================================================================

fn create_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let exp = (now + Duration::hours(JWT_EXPIRATION_HOURS)).timestamp();
    let iat = now.timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        exp,
        iat,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
}

fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

// ============================================================================
// Password Hashing
// ============================================================================

fn hash_password(password: &str) -> String {
    // In production, use bcrypt: bcrypt::hash(password, 12).unwrap()
    // For this example, using a simple approach. NEVER use in production!
    format!("hashed:{}", password)
}

fn verify_password(password: &str, hash: &str) -> bool {
    // In production, use bcrypt::verify(password, hash).unwrap_or(false)
    hash == format!("hashed:{}", password)
}

// ============================================================================
// Middleware for Authentication
// ============================================================================

#[derive(Clone)]
pub struct AuthUser {
    pub id: String,
}

#[axum::async_trait]
impl<S> axum::extract::FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;
        let auth_header = headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(ErrorResponse {
                        error: "Missing authorization header".to_string(),
                    }),
                )
                    .into_response()
            })?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(ErrorResponse {
                        error: "Invalid authorization header format".to_string(),
                    }),
                )
                    .into_response()
            })?;

        let claims = verify_jwt(token).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "Invalid or expired token".to_string(),
                }),
            )
                .into_response()
        })?;

        Ok(AuthUser { id: claims.sub })
    }
}

// ============================================================================
// Route Handlers
// ============================================================================

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<UserResponse>), (StatusCode, Json<ErrorResponse>)> {
    // Validation
    if payload.username.is_empty() || payload.password.len() < 6 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Username required, password must be at least 6 characters".to_string(),
            }),
        ));
    }

    let users = state.users.read().await;

    // Check if user already exists
    if users.iter().any(|u| u.username == payload.username) {
        return Err((
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                error: "Username already exists".to_string(),
            }),
        ));
    }

    drop(users); // Release read lock

    let user_id = uuid::Uuid::new_v4().to_string();
    let user = User {
        id: user_id.clone(),
        username: payload.username,
        email: payload.email,
        password_hash: hash_password(&payload.password),
    };

    let mut users = state.users.write().await;
    users.push(user.clone());

    Ok((
        StatusCode::CREATED,
        Json(UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
        }),
    ))
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ErrorResponse>)> {
    let users = state.users.read().await;

    let user = users
        .iter()
        .find(|u| u.username == payload.username)
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "Invalid username or password".to_string(),
                }),
            )
        })?;

    if !verify_password(&payload.password, &user.password_hash) {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "Invalid username or password".to_string(),
            }),
        ));
    }

    let token = create_jwt(&user.id).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to create token".to_string(),
            }),
        )
    })?;

    Ok(Json(AuthResponse {
        token,
        user: UserResponse {
            id: user.id.clone(),
            username: user.username.clone(),
            email: user.email.clone(),
        },
    }))
}

async fn me(
    auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<UserResponse>, (StatusCode, Json<ErrorResponse>)> {
    let users = state.users.read().await;

    let user = users.iter().find(|u| u.id == auth.id).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "User not found".to_string(),
            }),
        )
    })?;

    Ok(Json(UserResponse {
        id: user.id.clone(),
        username: user.username.clone(),
        email: user.email.clone(),
    }))
}

async fn get_user(
    Path(user_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<UserResponse>, (StatusCode, Json<ErrorResponse>)> {
    let users = state.users.read().await;

    let user = users.iter().find(|u| u.id == user_id).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "User not found".to_string(),
            }),
        )
    })?;

    Ok(Json(UserResponse {
        id: user.id.clone(),
        username: user.username.clone(),
        email: user.email.clone(),
    }))
}

async fn logout() -> Json<serde_json::Value> {
    Json(serde_json::json!({"message": "Logged out successfully"}))
}

async fn health() -> &'static str {
    "OK"
}

// ============================================================================
// Router Setup
// ============================================================================

#[tokio::main]
async fn main() {
    let state = AppState::new();

    // Build router
    let app = Router::new()
        // Public routes
        .route("/health", get(health))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        // Protected routes
        .route("/auth/me", get(me))
        .route("/auth/logout", post(logout))
        .route("/users/:id", get(get_user))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
