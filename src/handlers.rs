use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use serde::{Serialize};
use serde_json::json;

use crate::models::Level;
use crate::auth::AuthUser;

#[get("/users/{user_id}/levels")]
pub async fn get_levels_by_user(
    pool: web::Data<PgPool>,
    user_id: web::Path<String>,
) -> impl Responder {
    let uuid = match Uuid::parse_str(&user_id.into_inner()) {
        Ok(u) => u,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID"),
    };
    let rows = sqlx::query!(
        "SELECT levels.id, levels.name, users.username AS author, levels.official, levels.version FROM levels JOIN users ON levels.user_id = users.id WHERE levels.user_id = $1",
        uuid
    )
    .fetch_all(pool.get_ref())
    .await;

    match rows {
        Ok(records) => {
            let summaries: Vec<LevelSummary> = records.into_iter().map(|r| LevelSummary {
            id: r.id.to_string(),
                name: r.name,
                author: Some(r.author),
                official: r.official,
                version: r.version,
            }).collect();
            HttpResponse::Ok().json(summaries)
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Query failed: {}", e)),
    }
}

#[post("/levels")]
pub async fn add_level(
    pool: web::Data<PgPool>,
    user: AuthUser,
    level: web::Json<Level>,
) -> impl Responder {
    let level = level.into_inner();

    let user_uuid = match Uuid::parse_str(&user.user_id) {
        Ok(u) => u,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID"),
    };

    if let Some(id) = &level.id {
        // Update existing level
        let level_uuid = match Uuid::parse_str(id) {
            Ok(u) => u,
            Err(_) => return HttpResponse::BadRequest().body("Invalid Level UUID"),
        };

        let result = sqlx::query!(
            "UPDATE levels SET name = $1, description = $2, version = $3, total_crystals = $4, solution = $5, key = $6, map = $7, size = $8, spawn = $9 WHERE id = $10 AND user_id = $11 RETURNING id",
            level.metadata.name,
            level.metadata.description,
            level.metadata.version,
            level.metadata.total_crystals,
            &level.solution,
            &level.key,
            serde_json::to_value(&level.map).unwrap(),
            &level.size,
            &level.spawn,
            level_uuid,
            user_uuid
        )
        .fetch_optional(pool.get_ref())
        .await;

        match result {
            Ok(Some(record)) => HttpResponse::Ok().json(json!({"id": record.id.to_string()})),
            Ok(None) => HttpResponse::NotFound().body("Level not found or not owned by user"),
            Err(e) => HttpResponse::InternalServerError().body(format!("Update failed: {}", e)),
        }
    } else {
        // Insert new level
        let row = sqlx::query!(
            "INSERT INTO levels (name, description, commended, official, version, solution, key, map, size, spawn, user_id, total_crystals) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) RETURNING id",
            level.metadata.name,
            level.metadata.description,
            false,
            false,
            level.metadata.version,
            &level.solution,
            &level.key,
            serde_json::to_value(&level.map).unwrap(),
            &level.size,
            &level.spawn,
            user_uuid,
            level.metadata.total_crystals
        )
        .fetch_one(pool.get_ref())
        .await;

        match row {
            Ok(record) => HttpResponse::Ok().json(json!({"id": record.id.to_string()})),
            Err(e) => HttpResponse::InternalServerError().body(format!("Insert failed: {}", e)),
        }
    }
}

#[get("/levels/{id}")]
pub async fn get_level_by_id(
    pool: web::Data<PgPool>,
    id: web::Path<String>,
) -> impl Responder {
    
    let uuid = match Uuid::parse_str(&id.into_inner()) {
        Ok(u) => u,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID"),
    };
    println!("{:?}", sqlx::query!("SELECT json_build_object('key', key) FROM levels WHERE id = $1", uuid)
        .fetch_optional(pool.get_ref())
        .await);

    let result = sqlx::query!(r#"
        SELECT json_build_object(
            'key', levels.key,
            'map', levels.map,
            'metadata', json_build_object(
                'author_id', levels.user_id::text,
                'author_name', users.username,
                'description', levels.description,
                'name', levels.name,
                'commended', levels.commended,
                'id', levels.id::text,
                'official', levels.official,
                'version', levels.version
            ),
            'solution', levels.solution,
            'size', levels.size,
            'spawn', levels.spawn
        ) AS level_json
        FROM levels
        JOIN users ON levels.user_id = users.id
        WHERE levels.id = $1
    "#, uuid)
        .fetch_optional(pool.get_ref())
        .await;

    match result {
        Ok(Some(row)) => {
            HttpResponse::Ok().content_type("application/json").body(row.level_json.unwrap().to_string())
        },
        Ok(None) => HttpResponse::NotFound().body("Level not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Query error: {}", e)),
    }
}

#[derive(Serialize)]
struct LevelSummary {
    id: String,
    name: Option<String>,
    author: Option<String>,
    official: Option<bool>,
    version: Option<i32>,
}

#[get("/levels")]
pub async fn search_levels(
    pool: web::Data<PgPool>,
    params: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let search = params.get("search").map(|s| format!("%{}%", s)).unwrap_or_else(|| "%".to_string());
    let page: i64 = params.get("page").and_then(|p| p.parse().ok()).unwrap_or(1);
    let per_page: i64 = params.get("per_page").and_then(|p| p.parse().ok()).unwrap_or(20);
    let offset = (page - 1) * per_page;

    let rows = sqlx::query!(r#"
        SELECT json_build_object(
            'id', levels.id::text,
            'name', levels.name,
            'description', levels.description,
            'author_id', users.id::text,
            'author', users.username,
            'official', levels.official,
            'commended', levels.commended,
            'version', levels.version
        ) AS summary_json
        FROM levels JOIN users ON levels.user_id = users.id
        WHERE levels.name LIKE $1
        ORDER BY levels.name
        LIMIT $2 OFFSET $3
    "#, search, per_page, offset)
    .fetch_all(pool.get_ref())
    .await;

    match rows {
        Ok(records) => {
            let summaries: Vec<serde_json::Value> = records.into_iter().filter_map(|r| r.summary_json).collect();
            HttpResponse::Ok().json(summaries)
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Search failed: {}", e)),
    }
}

// Star level
#[post("/levels/{id}/star")]
pub async fn star_level(
    pool: web::Data<PgPool>,
    id: web::Path<String>,
    user: AuthUser
) -> impl Responder {
    let level_uuid = match Uuid::parse_str(&id.into_inner()) {
        Ok(u) => u,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID"),
    };

    let user_uuid = match Uuid::parse_str(&user.user_id) {
        Ok(u) => u,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID"),
    };

    let result = sqlx::query!(
        "INSERT INTO level_stars (user_id, level_id) VALUES ($1, $2) ON CONFLICT (user_id, level_id) DO NOTHING",
        user_uuid,
        level_uuid
    )
    .execute(pool.get_ref())
    .await;
    
    match result {
        Ok(res) => {
            if res.rows_affected() > 0 {
                HttpResponse::Ok().json(json!({"starred": true}))
            } else {
                HttpResponse::Conflict().body("Level already starred")
            }
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Insert failed: {}", e)),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(add_level)
       .service(get_level_by_id)
       .service(search_levels)
       .service(get_levels_by_user)
       .service(star_level);
}
