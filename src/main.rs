mod product;
mod random_products;

use std::time::Instant;
use axum::{
    routing::{get},
    response::IntoResponse, Router,
};
use axum::body::Body;
use axum::extract::State;
use axum::http::{Response, StatusCode};
use crate::product::Product;
use crate::random_products::create_random_products;


async fn get_products_json(State(state): State<AppState>) -> impl IntoResponse {
    let products = state.products;
    let start_time = Instant::now();
    let products_bytes = serde_json::to_vec(&products).unwrap();
    let elapsed_time = start_time.elapsed();
    println!("Time elapsed for serialization: {:?}", elapsed_time);

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(products_bytes))
        .unwrap()
}

#[derive(Clone)]
struct AppState {
    pub products: Vec<Product>,
}

#[tokio::main]
async fn main() {
    let initial_products = create_random_products();
    let app_state = AppState {
        products: initial_products,
    };

    let app = Router::new()
        .route("/json", get(get_products_json))
        .with_state(app_state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

