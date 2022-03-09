use crate::db;
use crate::errors::ApiError;
use crate::schema::users;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub handle: String,
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

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Identifiable)]
#[table_name = "users"]
pub struct User {
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

    pub fn create(new_user: NewUser) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = diesel::insert_into(users::table)
            .values(User::from(new_user))
            .get_result(&conn)?;

        Ok(user)
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

impl From<NewUser> for User {
    fn from(user: NewUser) -> Self {
        User {
            id: Uuid::new_v4(),
            name: user.name,
            handle: user.handle,
            created: Utc::now().naive_utc(),
        }
    }
}
