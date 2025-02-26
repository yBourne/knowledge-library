use axum::{extract::{State, Path}, Json, Router, routing::{get, post}};
use sqlx::PgPool;
use serde_json::json;
use uuid::Uuid;
use crate::models::{Article, CreateArticle};

pub async fn health_check() -> Json<serde_json::Value> {
    Json(json!({"status": "ok"}))
}

pub async fn create_article(
    State(pool): State<PgPool>,
    Json(data): Json<CreateArticle>,
) -> Json<Article> {
    let article = sqlx::query_as!(
        Article,
        "INSERT INTO articles (id, user_id, title, content, tags) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        Uuid::new_v4(), Uuid::new_v4(), data.title, data.content, &data.tags
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to insert article");

    Json(article)
}

pub async fn get_articles(State(pool): State<PgPool>) -> Json<Vec<Article>> {
    let articles = sqlx::query_as!(Article, "SELECT * FROM articles")
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch articles");

    Json(articles)
}

pub fn create_routes() -> Router<PgPool> {
    Router::new()
        .route("/health", get(health_check))
        .route("/articles", post(create_article).get(get_articles))
}
