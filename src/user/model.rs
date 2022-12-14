use crate::db;
use crate::error_handler::CustomError;
use crate::schema::users;
use diesel::prelude::*;
use pwhash::{bcrypt, unix};
use serde::{Deserialize, Serialize};
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
    pub fn basic_auth(user: User) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let user_db = users::table
            .filter(users::username.eq(user.username))
            .first::<Users>(&conn)?;
        // let b = x;
        let a = user_db.clone();
        let is_valid = unix::verify(user.password.unwrap(), &a.password.unwrap());
        if is_valid {
            Ok(user_db)
        } else {
            Ok(Self {
                id: 1,
                username: Some("notFound".to_string()),
                password: Some("Wrong".to_string()),
            })
        }
    }
}
