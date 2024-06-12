use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    serve, Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::Ipv4Addr,
    sync::{Arc, RwLock},
};
use tokio::net::TcpListener;

type Db = Arc<RwLock<HashMap<String, Movie>>>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Movie {
    id: String,
    name: String,
    year: u16,
    was_good: bool,
}

async fn get_movie(
    Path(movie_id): Path<String>,
    State(state): State<Db>,
) -> Result<impl IntoResponse, StatusCode> {
    let movies = state.read().unwrap();
    let movie = movies
        .get(&movie_id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(movie.clone()))
}

async fn store_movie(
    State(state): State<Db>,
    extract::Json(movie): extract::Json<Movie>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut movies = state.write().unwrap();
    movies.insert(movie.id.clone(), movie.clone());
    Ok(Json(movie.clone()))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/movie/:id", get(get_movie))
        .route("/movie", post(store_movie))
        .with_state(Db::default());
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 8000))
        .await
        .unwrap();
    serve(listener, app.into_make_service()).await.unwrap();
}
