use crate::api_error::ApiError;
use crate::db;
use crate::schema::users;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct UserDto {
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

    pub fn create(user: UserDto) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = User::from(user);
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(&conn)?;

        Ok(user)
    }

    pub fn update(id: Uuid, user: UserDto) -> Result<Self, ApiError> {
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

impl From<UserDto> for User {
    fn from(user: UserDto) -> Self {
        User {
            id: Uuid::new_v4(),
            name: user.name,
            handle: user.handle,
            created: Utc::now().naive_utc(),
        }
    }
}
