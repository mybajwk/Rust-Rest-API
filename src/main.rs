#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{
    dev::ServiceRequest,
    error::Error,
    web::{self, Data},
    App, HttpMessage, HttpServer,
};
use actix_web_httpauth::{
    extractors::{
        bearer::{self, BearerAuth},
        AuthenticationError,
    },
    middleware::HttpAuthentication,
};
use chrono::Utc;
use dotenv::dotenv;
use hmac::{Hmac, Mac};
use jsonwebtoken::{decode, DecodingKey, Validation};
use jwt::VerifyWithKey;
use listenfd::ListenFd;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::env;

mod absence;
mod db;
mod employees;
mod error_handler;
mod schema;
mod user;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenClaims {
    sub: String,
    iat: i64,
    exp: i64,
}

async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set!");
    // let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();
    let token_string = credentials.token();

    // let claims: Result<TokenClaims, &str> = token_string
    //     .verify_with_key(&key)
    //     .map_err(|_| "Invalid token");
    // println!("{}", claims.clone().unwrap().id);
    let claims = decode::<TokenClaims>(
        &token_string,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    );
    // .unwrap();
    // println!("{:?}", claims);
    match claims {
        Ok(value) => {
            if value.claims.exp > Utc::now().timestamp_micros() {
                req.extensions_mut().insert(value.claims);
                Ok(req)
            } else {
                let config = req
                    .app_data::<bearer::Config>()
                    .cloned()
                    .unwrap_or_default()
                    .scope("");

                Err((AuthenticationError::from(config).into(), req))
            }
            // println!("{:?}", value.claims);
        }
        Err(_) => {
            let config = req
                .app_data::<bearer::Config>()
                .cloned()
                .unwrap_or_default()
                .scope("");

            Err((AuthenticationError::from(config).into(), req))
        }
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    db::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        let bearer_middleware = HttpAuthentication::bearer(validator);
        App::new()
            .configure(employees::init_routes)
            .configure(user::init_routes)
            .service(
                web::scope("")
                    .wrap(bearer_middleware)
                    .configure(absence::init_routes),
            )
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set host in .env");
            let port = env::var("PORT").expect("Please set port in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.run().await
}
