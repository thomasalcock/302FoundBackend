use std::time::SystemTime;

use crate::{app_state::AppState, error::{Error, Result}};

use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Location {
    id: u64,
    user_id: u64,
    timestamp: u64,
    latidute: f64,
    longitude: f64
}

#[derive(Deserialize)]
pub struct LocationForCreate {
    user_id: u64,
    latitude: f64,
    longitude: f64,
}

pub trait LocationStore {
    async fn create_location(&self, location: LocationForCreate) -> Result<()>;
    async fn get_location(&self) -> Result<Vec<Location>>;
    async fn location_by_id(&self, id: u64) -> Result<Location>;
    async fn delete_location(&self, id: u64) -> Result<()>;
}

impl LocationStore for AppState {

    async fn create_location(&self, location: LocationForCreate) -> Result<()> {
        query("INSERT INTO location(user_id, timestamp, latitude, longitude ) VALUES ($1, $2, $3, $4)")
            .bind(location.user_id.to_string())
            .bind(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
                .map_err(|_| Error::UnknownError)?.as_secs().to_string())
            .bind(location.latitude.to_string())
            .bind(location.longitude.to_string())
            .fetch_optional(self.conn())
            .await
            .map_err(|_| Error::DatabaseError)?;
        Ok(())
    }

    async fn get_location(&self) -> Result<Vec<Location>> {
        let results = query_as("SELECT * FROM location")
            .fetch_all(self.conn())
            .await
            .map_err(|_| Error::DatabaseError)?;
        Ok(results)
    }


    async fn location_by_id(&self, id: u64) -> Result<Location> {
        let results = query_as("SELECT * FROM location WHERE id = $1")
            .bind(id.to_string())
            .fetch_one(self.conn())
            .await
            .map_err(|_| Error::DatabaseError)?;
        Ok(results)
    }

    async fn delete_location(&self, id: u64) -> Result<()> {
        query("REMOVE FROM location WHERE id = $1")
            .bind(id.to_string())
            .fetch_optional(self.conn())
            .await
            .map_err(|_| Error::DatabaseError)?;
        Ok(())

    }
}
