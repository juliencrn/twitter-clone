use crate::api_error::ApiError;
use crate::db;
use crate::schema::users;
use argon2::Config;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct CreateUserDto {
    pub name: String,
    pub handle: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct UpdateUserDto {
    pub name: String,
    pub handle: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub name: String,   // Mary
    pub handle: String, // @logiconly9 (unique)
    pub created: NaiveDateTime,

    // Don't send it to the user
    #[diesel(skip_deserializing)]
    pub password: String,
}

pub struct UserPub {
    pub id: Uuid,
    pub name: String,   // Mary
    pub handle: String, // @logiconly9 (unique)
    pub created: NaiveDateTime,
}

impl User {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let users = users::table.load::<User>(&conn)?;

        Ok(users)
    }

    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = users::table.filter(users::id.eq(id)).first(&conn)?;

        Ok(user)
    }

    pub fn create(user: CreateUserDto) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let mut user = User::from(user);
        user.hash_password()?;

        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(&conn)?;

        Ok(user)
    }

    pub fn hash_password(&mut self) -> Result<(), ApiError> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();
        let pwd = self.password.as_bytes();

        self.password = argon2::hash_encoded(pwd, &salt, &config)
            .map_err(|e| ApiError::new(500, format!("Failed to hash password: {}", e)))?;

        Ok(())
    }

    pub fn verify_password(&self, password: &[u8]) -> Result<bool, ApiError> {
        argon2::verify_encoded(&self.password, password)
            .map_err(|e| ApiError::new(500, format!("Failed to verify password: {}", e)))
    }

    pub fn update(id: Uuid, user: UpdateUserDto) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(user)
            .get_result(&conn)?;

        Ok(user)
    }

    pub fn delete(id: Uuid) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = diesel::delete(users::table.filter(users::id.eq(id))).get_result(&conn)?;

        Ok(user)
    }
}

impl From<CreateUserDto> for User {
    fn from(user: CreateUserDto) -> Self {
        User {
            id: Uuid::new_v4(),
            name: user.name,
            handle: user.handle,
            created: Utc::now().naive_utc(),
            password: user.password,
        }
    }
}
