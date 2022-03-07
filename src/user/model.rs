use crate::db;
use crate::errors::ApiError;
use crate::schema::users;
use argon2::Config;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct NewUser {
    #[validate(length(
        min = 3,
        message = "name is required and must be at least 3 characters"
    ))]
    pub name: String,
    #[validate(length(
        min = 3,
        message = "handle is required and must be at least 3 characters"
    ))]
    pub handle: String,
    #[validate(length(
        min = 6,
        message = "password is required and must be at least 6 characters"
    ))]
    pub password: String,
}

#[derive(Serialize, Deserialize, AsChangeset, Validate)]
#[table_name = "users"]
pub struct UpdateUser {
    #[validate(length(
        min = 3,
        message = "name is required and must be at least 3 characters"
    ))]
    pub name: String,
    #[validate(length(
        min = 3,
        message = "handle is required and must be at least 3 characters"
    ))]
    pub handle: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub name: String,   // Mary
    pub handle: String, // @logiconly9 (unique)
    pub created: NaiveDateTime,
    pub password: String,
}

// TODO: Split account { password } from user profile and remove below
#[derive(Serialize, Deserialize)]
pub struct PublicUser {
    pub id: Uuid,
    pub name: String,
    pub handle: String,
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

    pub fn find_by_handle(handle: &str) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = users::table.filter(users::handle.eq(handle)).first(&conn)?;

        Ok(user)
    }

    pub fn create(user: NewUser) -> Result<Self, ApiError> {
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

    pub fn verify_password(&self, password: &str) -> Result<bool, ApiError> {
        argon2::verify_encoded(&self.password, password.as_bytes())
            .map_err(|e| ApiError::new(500, format!("Failed to verify password: {}", e)))
    }

    pub fn update(id: Uuid, user: UpdateUser) -> Result<Self, ApiError> {
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

// TODO: Split account { password } from user profile and remove below
impl From<NewUser> for User {
    fn from(user: NewUser) -> Self {
        User {
            id: Uuid::new_v4(),
            name: user.name,
            handle: user.handle,
            created: Utc::now().naive_utc(),
            password: user.password,
        }
    }
}

impl From<User> for PublicUser {
    fn from(user: User) -> Self {
        PublicUser {
            id: user.id,
            handle: user.handle,
            name: user.name,
            created: user.created,
        }
    }
}
