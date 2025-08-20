use actix_web::{web, HttpResponse, Responder, post, HttpRequest};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Serialize};
use actix_web::dev::Payload;
use actix_web::{FromRequest, Error};
use futures_util::future::{ready, Ready};
use argon2::{Argon2, PasswordHash, PasswordVerifier, PasswordHasher};
use argon2::password_hash::SaltString;
use rand_core::OsRng;

#[derive(Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug)]
pub struct AuthUser {
    pub user_id: String,
}

impl FromRequest for AuthUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(token) = auth_header.to_str() {
                if let Some(token) = token.strip_prefix("Bearer ") {
                    let key = DecodingKey::from_secret("secret".as_ref());
                    if let Ok(data) = decode::<Claims>(token, &key, &Validation::default()) {
                        return ready(Ok(AuthUser { user_id: data.claims.sub }));
                    }
                }
            }
        }
        ready(Err(actix_web::error::ErrorUnauthorized("Invalid token")))
    }
}

#[post("/signup")]
pub async fn signup(pool: web::Data<PgPool>, data: web::Json<AuthRequest>) -> impl Responder {
    // Generate random salt correctly
    let salt = SaltString::generate(&mut OsRng);

    // Hash password using Argon2 and SaltString
    let argon2 = Argon2::default();
    let password_hash = match argon2.hash_password(data.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(_) => return HttpResponse::InternalServerError().body("Password hashing failed"),
    };

    let id = Uuid::new_v4();

    let result = sqlx::query(
        "INSERT INTO users (id, username, password_hash, is_admin) VALUES ($1, $2, $3, $4)"
    )
    .bind(id)
    .bind(&data.username)
    .bind(&password_hash)
    .bind(false)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(json!({ "id": id.to_string() })),
        Err(e) => {
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.constraint() == Some("users_username_key") {
                    return HttpResponse::BadRequest().body("Username already exists");
                }
            }
            HttpResponse::InternalServerError().body(format!("Signup error: {}", e))
        }
    }
}

#[post("/login")]
pub async fn login(pool: web::Data<PgPool>, data: web::Json<AuthRequest>) -> impl Responder {
    let row = sqlx::query!("SELECT id, password_hash FROM users WHERE username = $1", &data.username)
        .fetch_optional(pool.get_ref())
        .await;

    let user = match row {
        Ok(Some(u)) => u,
        _ => return HttpResponse::Unauthorized().body("Invalid credentials"),
    };

    let argon2 = Argon2::default();

    let parsed_hash = match PasswordHash::new(&user.password_hash) {
        Ok(ph) => ph,
        Err(_) => return HttpResponse::InternalServerError().body("Invalid password hash stored"),
    };

    if argon2.verify_password(data.password.as_bytes(), &parsed_hash).is_ok() {
        let claims = Claims {
            sub: user.id.to_string(),
            //30 day expiration
            exp: chrono::Utc::now().timestamp() as usize + 30 * 24 * 60 * 60,
        };
        let token = match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("secret".as_ref()),
        ) {
            Ok(t) => t,
            Err(_) => return HttpResponse::InternalServerError().body("Token generation failed"),
        };

        return HttpResponse::Ok().json(json!({ "token": token }));
    }

    HttpResponse::Unauthorized().body("Invalid credentials")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(signup).service(login);
}
