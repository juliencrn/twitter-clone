use crate::db;
use crate::errors::ApiError;
use crate::schema::user_accounts;
use argon2::Config;
use diesel::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "user_accounts"]
pub struct UserAccount {
    pub id: Uuid,
    pub email: String,
    pub user_id: Uuid,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewUserAccount {
    pub id: Uuid,
    pub email: String,
    pub user_id: Uuid,
    pub password: String,
}

impl UserAccount {
    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let result = user_accounts::table
            .filter(user_accounts::id.eq(id))
            .first::<UserAccount>(&conn)?;

        Ok(result)
    }

    pub fn find_by_user(user_id: Uuid) -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let result = user_accounts::table
            .filter(user_accounts::user_id.eq(user_id))
            .load::<UserAccount>(&conn)?;

        Ok(result)
    }

    pub fn create(new_account: NewUserAccount) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let mut account = UserAccount::from(new_account);

        account.hash_password()?;

        let account = diesel::insert_into(user_accounts::table)
            .values(&account)
            .get_result::<UserAccount>(&conn)?;

        Ok(account)
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

    pub fn delete(id: Uuid) -> Result<UserAccount, ApiError> {
        let conn = db::connection()?;

        let account =
            diesel::delete(user_accounts::table.find(id)).get_result::<UserAccount>(&conn)?;

        Ok(account)
    }
}

impl From<NewUserAccount> for UserAccount {
    fn from(account: NewUserAccount) -> Self {
        UserAccount {
            id: Uuid::new_v4(),
            email: account.email,
            user_id: account.user_id,
            password: account.password,
        }
    }
}
