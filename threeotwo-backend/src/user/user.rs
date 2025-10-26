use crate::{error::{Error, Result}, AppState};

use axum::extract::Path;
use serde::Deserialize;
use sqlx::{Row, query};

pub struct User {
    user_name: String,
    full_name: String,
    email: String,
    phone_numer: String
}



#[derive(Deserialize)]
pub struct UserForCreate {
    full_name: String
}

#[derive(Deserialize)]
pub struct UserForUpdate {
    user_name: Option<String>,
    full_name: Option<String>,
    phone_numer: Option<String>,
    email: Option<String>,
    alias: Option<u64>
}

pub trait UserStore {
    async fn create_user(&mut self, user: UserForCreate) -> Result<()>;
    async fn update_user(&mut self, user_id: u64, user: UserForUpdate) -> Result<()>;
    async fn users(&self) -> Result<Vec<(u64, User)>>;
    async fn user_by_id(&self, user_id: u64) -> Result<User>; 
    async fn delete_user(&mut self, user_id: u64) -> Result<()>;
}

impl UserStore for AppState {
    async fn create_user(&mut self, user: UserForCreate) -> Result<()> {
        println!("INSERTING INTO USERS");

        query(
            format!("INSERT INTO TABLE users(full_name) VALUES {}",
            user.full_name
            ).as_str())
            .fetch_one(self.conn())
            .await
            .map_err(|_| Error::DatabaseError)?;

        Ok(())

    }

    async fn update_user(&mut self, id: u64, _user: UserForUpdate) -> Result<()> {
        println!("UPDATING USER({})", id);

        todo!()

    }

    async fn users(&self) -> Result<Vec<(u64,User)>> {
        println!("READING ALL USERS");
        todo!()
    }

    async fn user_by_id(&self, user_id: u64) -> Result<User> {
        println!("READING USER({})", user_id);
        todo!()
    }

    async fn delete_user(&mut self, _user_id: u64) -> Result<()> {todo!()}
}
