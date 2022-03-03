use crate::api_error::ApiError;
use crate::db;
use crate::schema::users;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct UserDto {
    pub name: String,
    pub handle: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
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

    pub fn find(handle: &str) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = users::table.filter(users::handle.eq(handle)).first(&conn)?;

        Ok(user)
    }

    pub fn create(user: UserDto) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = User::from(user);
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(&conn)?;

        Ok(user)
    }

    pub fn update(handle: &str, user: UserDto) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = diesel::update(users::table)
            .filter(users::handle.eq(handle))
            .set(user)
            .get_result(&conn)?;

        Ok(user)
    }

    pub fn delete(handle: &str) -> Result<usize, ApiError> {
        let conn = db::connection()?;

        let res = diesel::delete(users::table.filter(users::handle.eq(handle))).execute(&conn)?;

        Ok(res)
    }
}

impl From<UserDto> for User {
    fn from(user: UserDto) -> Self {
        User {
            name: user.name,
            handle: user.handle,
            created: Utc::now().naive_utc(),
        }
    }
}
