use actix_web::{delete, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::auth::AuthUser;
use uuid::Uuid;

// Helper function to check admin status (requires user_id and pool)
async fn is_admin(user_id: &str, pool: &PgPool) -> Result<bool, sqlx::Error> {
    let uuid = match Uuid::parse_str(user_id) {
        Ok(u) => u,
        Err(_) => return Ok(false),
    };
    let row = sqlx::query!("SELECT is_admin FROM users WHERE id = $1", uuid)
        .fetch_optional(pool)
        .await?;
    Ok(row.map(|r| r.is_admin).unwrap_or(false))
}

#[delete("/levels/{id}")]
pub async fn delete_level(
    pool: web::Data<PgPool>,
    user: AuthUser,
    id: web::Path<String>,
) -> impl Responder {
    if !is_admin(&user.user_id, pool.get_ref()).await.unwrap_or(false) {
        return HttpResponse::Forbidden().body("Admin privileges required");
    }
    let uuid = match Uuid::parse_str(&id.into_inner()) {
        Ok(u) => u,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID"),
    };
    let result = sqlx::query!("DELETE FROM levels WHERE id = $1", uuid)
        .execute(pool.get_ref())
        .await;
    match result {
        Ok(_) => HttpResponse::Ok().body("Level deleted"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Delete failed: {}", e)),
    }
}

#[post("/levels/{id}/commend")]
pub async fn commend_level(
    pool: web::Data<PgPool>,
    user: AuthUser,
    id: web::Path<String>,
) -> impl Responder {
    if !is_admin(&user.user_id, pool.get_ref()).await.unwrap_or(false) {
        return HttpResponse::Forbidden().body("Admin privileges required");
    }
    let uuid = match Uuid::parse_str(&id.into_inner()) {
        Ok(u) => u,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID"),
    };
    let result = sqlx::query!("UPDATE levels SET official = true WHERE id = $1", uuid)
        .execute(pool.get_ref())
        .await;
    match result {
        Ok(_) => HttpResponse::Ok().body("Level commended (marked official)"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Commend failed: {}", e)),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(delete_level)
       .service(commend_level);
}
