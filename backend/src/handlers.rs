use crate::api::{fetch_crypto_data, fetch_news_data};
use crate::db::{get_from_cache, init_db, save_to_cache};
use actix_web::{get, web, HttpResponse, Responder};

#[get("/crypto/{symbol}")]
pub async fn get_crypto_data(symbol: web::Path<String>) -> impl Responder {
    let symbol = symbol.into_inner().to_uppercase();
    let conn = match init_db() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to open cache"),
    };

    let cache_key = format!("crypto-{}", symbol);
    if let Some(cached_json) = get_from_cache(&conn, &cache_key, 5) {
        return HttpResponse::Ok()
            .content_type("application/json")
            .body(cached_json);
    }

    match fetch_crypto_data().await {
        Ok(crypto_list) => {
            let crypto = crypto_list
                .iter()
                .find(|c| c.symbol.to_uppercase() == symbol);

            match crypto {
                Some(data) => {
                    let json = match serde_json::to_string(data) {
                        Ok(j) => j,
                        Err(_) => {
                            return HttpResponse::InternalServerError().body("Serialization error")
                        }
                    };
                    let _ = save_to_cache(&conn, &cache_key, &json);
                    HttpResponse::Ok().json(data)
                }
                None => HttpResponse::NotFound().body("Cryptocurrency not found"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch crypto data"),
    }
}

#[get("/news/{symbol}")]
pub async fn get_crypto_news(symbol: web::Path<String>) -> impl Responder {
    let symbol = symbol.into_inner().to_uppercase();
    let conn = match init_db() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to open cache"),
    };

    let news_key = format!("news-{}", symbol);
    if let Some(cached_news) = get_from_cache(&conn, &news_key, 10) {
        return HttpResponse::Ok()
            .content_type("application/json")
            .body(cached_news);
    }

    match fetch_crypto_data().await {
        Ok(crypto_list) => {
            let crypto = crypto_list
                .iter()
                .find(|c| c.symbol.to_uppercase() == symbol);

            match crypto {
                Some(coin) => {
                    let name = &coin.name;
                    match fetch_news_data(name).await {
                        Ok(news) => {
                            let json = match serde_json::to_string(&news) {
                                Ok(j) => j,
                                Err(_) => {
                                    return HttpResponse::InternalServerError()
                                        .body("Serialization error")
                                }
                            };
                            let _ = save_to_cache(&conn, &news_key, &json);
                            HttpResponse::Ok().json(news)
                        }
                        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch news"),
                    }
                }
                None => HttpResponse::NotFound().body("Cryptocurrency not found"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch crypto data"),
    }
}
