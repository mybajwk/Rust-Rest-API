use std::env;

use crate::error_handler::CustomError;
use crate::schema::users;
use crate::{db, TokenClaims};
use chrono::{Duration, Utc};
use diesel::prelude::*;
use hmac::{Hmac, Mac};
use jsonwebtoken::{encode, EncodingKey, Header};
use jwt::SignWithKey;
use pwhash::{bcrypt, unix};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "users"]
pub struct User {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Clone)]
#[table_name = "users"]
pub struct Users {
    pub id: i32,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Users {
    pub fn create(user: User) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let mut user = User::from(user);

        // let hash = bcrypt::hash(user.password).unwrap();
        let hash = bcrypt::hash(user.password.unwrap()).unwrap();

        user.password = Some(hash);

        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(&conn)?;
        Ok(user)
    }
    pub fn basic_auth(user: User) -> Result<String, CustomError> {
        let jwt_secret = env::var("JWT_SECRET").unwrap();

        let conn = db::connection()?;
        let user_db = users::table
            .filter(users::username.eq(user.username))
            .first::<Users>(&conn)?;
        // let b = x;
        let a = user_db.clone();
        let is_valid = unix::verify(user.password.unwrap(), &a.password.unwrap());
        if is_valid {
            let iat = Utc::now();
            let exp = iat + Duration::days(1);
            let claims = TokenClaims {
                // id: user_db.id,
                sub: user_db.username.unwrap(),
                iat: iat.timestamp_micros(),
                exp: exp.timestamp_micros(),
            };
            // let token_str = claims.sign_with_key(&jwt_secret).unwrap();
            let token_str = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(jwt_secret.as_bytes()),
            )
            .unwrap();
            Ok(token_str)
        } else {
            Ok("Wrong pass".to_string())
        }
    }
}
