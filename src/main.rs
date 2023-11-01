mod product;
mod random_products;

use std::{
    io::{Read, Write},
    time::Instant,
};
use axum::{
    extract::State,
    body::Body,
    routing::{get},
    response::IntoResponse,
    Router,
    http::{Response, StatusCode},
};
use brotli::CompressorReader;
use rkyv::{
    ser::Serializer,
    ser::serializers::AllocSerializer,
};
use zstd::encode_all;
use crate::product::Product;
use crate::random_products::create_random_products;
use flate2::{
    write::GzEncoder,
    Compression,
};

async fn compress_and_log(data: &[u8], name: &str) {
    // gzip
    let start_time = Instant::now();
    let mut gz = GzEncoder::new(Vec::new(), Compression::default());
    gz.write_all(data).unwrap();
    let compressed_gzip = gz.finish().unwrap();
    let elapsed_time = start_time.elapsed();
    println!("Time elapsed for {} gzip compression: {:?}", name, elapsed_time);
    println!("Size after {} gzip compression: {} bytes", name, compressed_gzip.len());

    // brotli
    let start_time = Instant::now();
    let mut compressed_brotli = Vec::new();
    let mut compressor = CompressorReader::new(data, 4096, 11, 22);
    compressor.read_to_end(&mut compressed_brotli).unwrap();
    let elapsed_time = start_time.elapsed();
    println!("Time elapsed for {} brotli compression: {:?}", name, elapsed_time);
    println!("Size after {} brotli compression: {} bytes", name, compressed_brotli.len());

    // zstd
    let start_time = Instant::now();
    let compressed_zstd = encode_all(data, 0).unwrap();
    let elapsed_time = start_time.elapsed();
    println!("Time elapsed for {} zstd compression: {:?}", name, elapsed_time);
    println!("Size after {} zstd compression: {} bytes", name, compressed_zstd.len());
}

async fn get_products_json(State(state): State<AppState>) -> impl IntoResponse {
    let products = state.products;
    let start_time = Instant::now();
    let products_bytes = serde_json::to_vec(&products).unwrap();
    println!();
    println!("Time elapsed for serialization: {:?}", start_time.elapsed());
    println!("Size of products_bytes (JSON): {} bytes", products_bytes.len());

    compress_and_log(&products_bytes, "JSON").await;

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(products_bytes))
        .unwrap()
}

async fn get_products_rkyv(State(state): State<AppState>) -> impl IntoResponse {
    let products = state.products.clone();
    let start_time = Instant::now();
    let mut serializer = AllocSerializer::<4096>::default();
    serializer.serialize_value(&products).unwrap();
    let products_bytes = serializer.into_serializer().into_inner().to_vec();
    println!();
    println!("Time elapsed for serialization: {:?}", start_time.elapsed());
    println!("Size of products_bytes (rkyv): {} bytes", products_bytes.len());

    compress_and_log(&products_bytes, "rkyv").await;

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/octet-stream")
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
        .route("/rkyv", get(get_products_rkyv))
        .with_state(app_state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

