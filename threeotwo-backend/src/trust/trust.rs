use crate::{app_state::AppState, error::{Error, Result}};

use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow};


#[derive(Debug,Serialize,Deserialize, FromRow)]
pub struct Trust {
    user: u64,
    trustee: u64
}


impl Trust {
    pub fn user(&self) -> u64 {self.user}
    pub fn trustee(&self) -> u64 {self.trustee}
}

pub trait TrustStore {
    async fn create_trust(&self, user: u64, trustee: u64) -> Result<()>;
    async fn get_trusts(&self) -> Result<Vec<Trust>>;
    async fn delete_trust(&self, user: u64, trustee: u64) -> Result<()>;
}

impl TrustStore for AppState {
    async fn create_trust(&self, user: u64, trustee : u64) -> Result<()> {
        query("INSERT INTO trust(user, trustee) VALUES ($1, $2)")
            .bind(user.to_string())
            .bind(trustee.to_string())
            .fetch_optional(self.conn())
            .await
            .map_err(|_| Error::DatabaseError)?;
        Ok(())
    }

    async fn get_trusts(&self) -> Result<Vec<Trust>> {
        let results = query_as("SELECT * FROM trust")
            .fetch_all(self.conn())
            .await
            .map_err(|_| Error::DatabaseError)?;
        Ok(results)
    }

    async fn delete_trust(&self, user: u64, trustee: u64) -> Result<()> {
        query("INSERT INTO trust(user, trustee) VALUES ($1, $2)")
            .bind(user.to_string())
            .bind(trustee.to_string())
            .fetch_optional(self.conn())
            .await
            .map_err(|_| Error::DatabaseError)?;
        Ok(())

    }
}
