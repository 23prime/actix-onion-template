use actix_cors::Cors;
use actix_web::http::{Method, header};

pub fn cors(allowed_origins: &[String]) -> Cors {
    let cors = Cors::default()
        .allowed_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allowed_headers(vec![
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
        ])
        // Cache preflight response for 1 hour
        .max_age(3600);

    allowed_origins
        .iter()
        .fold(cors, |cors, origin| cors.allowed_origin(origin))
}
