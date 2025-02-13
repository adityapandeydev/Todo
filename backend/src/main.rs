use axum::{extract::{Path, State}, response::Redirect, routing::{get, post}, Form, Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use std::net::SocketAddr;
use axum_error::Result;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv();
    let url = std::env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&url).await?;

    let app = Router::new()
        .route("/", get(list))
        .route("/create", post(create))
        .route("/read/:id", get(read))
        .route("/update", get(update))
        .route("/delete/:id", post(delete))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());

    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    Ok(axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?)
}

#[derive(Deserialize)]
struct NewTodo {
    description: String,
}

#[derive(Serialize, Deserialize)]
struct Todo {
    id: i32,
    description: String,
    done: bool,
}

async fn list(State(pool): State<PgPool>) -> Result<Json<Vec<Todo>>> {
    // List all notes
    let todos = sqlx::query_as!(Todo, "SELECT id, description, done FROM todos ORDER BY id")
        .fetch_all(&pool)
        .await?;
    Ok(Json(todos))
}

async fn create(State(pool): State<PgPool>, Form(todo): Form<NewTodo>) -> Result<Redirect> {
    // Create new note
    sqlx::query!(
        "INSERT INTO todos (description) VALUES ($1)",
        todo.description,
    )
    .execute(&pool)
    .await?;
    Ok(Redirect::to("http://localhost:5173"))
}

async fn read(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<Json<Todo>> {
    // Read todo
    let todo = sqlx::query_as!(
        Todo,
        "SELECT id, description, done FROM todos WHERE id = $1",
        id
    )
    .fetch_one(&pool)
    .await?;
    Ok(Json(todo))
}

async fn update(State(pool): State<PgPool>, Form(todo): Form<Todo>) -> Result<Redirect> {
    // Update todo
    sqlx::query!(
        "UPDATE todos SET description = $1, DONE = $2 WHERE id = $3",
        todo.description,
        todo.done,
        todo.id
    ).execute(&pool).await?;
    Ok(Redirect::to("http://localhost:5173"))
}

async fn delete(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<Redirect> {
    // Update todo
    sqlx::query!("DELETE FROM todos WHERE id = $1", id)
        .execute(&pool)
        .await?;
    Ok(Redirect::to("http://localhost:5173"))
}