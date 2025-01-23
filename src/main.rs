use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use actix_cors::Cors;
use crate::db::AppState;
use crate::payload::{ShortenResponse, Longurl};
use serde::Deserialize;
use url_shortener::base62_encode;
use num_bigint::{BigUint, ToBigUint};
mod payload;
mod model;
mod db;

#[derive(Deserialize)]
pub struct User {
    pub name: String,
    pub age: u8,
}

#[derive(Deserialize)]
pub struct ShortenerRequest {
    pub url: String,
}

#[get("/")]
async fn landing() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

async fn redirect(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let short_url = path.into_inner();
    if let Some(url_mapping) = data.retreive(short_url).await {
        HttpResponse::Found()
            .append_header(("Location", url_mapping.long_url))
            .finish()
    } else {
        HttpResponse::NotFound().body("URL Not Found!")
    }
}

async fn shortener(data: web::Data<AppState>, json: web::Json<Longurl>) -> impl Responder {
    let url = &json.into_inner().long_url;
    let bytes = url.as_bytes();
    
    // converting byte array to large integer
    let mut res: BigUint = 0.to_biguint().unwrap();

    for &byte in bytes {
        res = res * 256.to_biguint().unwrap() + (byte.to_biguint().unwrap());
    }
    
    let res = base62_encode(res);
    let short_url = format!("http://localhost:8080/{}", res);
    data.add(res.clone(), url.clone()).await;

    HttpResponse::Ok().json(ShortenResponse { short_url: short_url, })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let appstate: Data<AppState> = web::Data::new(AppState::new().await);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .app_data(appstate.clone())
            .wrap(cors)
            .route("/generate", web::post().to(shortener))
            .route("/{short_url}", web::get().to(redirect))
            .service(landing)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
