use crate::{error::{Error, Result}, location::location::Location, AppState};

use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow};

#[derive(Deserialize)]
pub struct UserForCreate {
    full_name: String
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct User {
    user_name: Option<String>,
    full_name: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    alias: Option<u64>
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct DBUser {
    id: u64,
    user_name: Option<String>,
    full_name: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    alias: Option<u64>
}



pub trait UserStore {
    async fn create_user(&mut self, user: UserForCreate) -> Result<()>;
    async fn update_user(&mut self, user_id: u64, user: User) -> Result<()>;
    async fn users(&self) -> Result<Vec<DBUser>>;
    async fn user_by_id(&self, user_id: u64) -> Result<User>; 
    async fn my_trustees(&self, user_id: u64) -> Result<Vec<DBUser>>;
    async fn my_locations(&self, user_id: u64) -> Result<Vec<Location>>;
    async fn delete_user(&self, user_id: u64) -> Result<()>;
}

impl AppState {
    async fn update_var(&self, id: u64, variable_name: String, value: Option<String>) -> Result<()> {
        let value = match value {
            None => return Ok(()),
            Some(v) => v
        };

        query("UPDATE user SET $1 = $2 WHERE user.id = $3")
            .bind(variable_name)
            .bind(value)
            .bind(id.to_string()) 
            .fetch_all(self.conn())
            .await
            .map_err(|_| Error::DatabaseError)?;
        Ok(())
    }


}

impl UserStore for AppState {
    async fn create_user(&mut self, user: UserForCreate) -> Result<()> {
        println!("INSERTING INTO USERS");

        query("INSERT INTO user(full_name) VALUES ('john')")   
            .bind(user.full_name)
            .fetch_one(self.conn())
            .await
            .map_err(|e| {println!("ERROR: {}", e); Error::DatabaseError})?;

        println!("INSERT SUCCESSFULL");
        Ok(())

    }

    async fn update_user(&mut self, id: u64, user: User) -> Result<()> {
        println!("UPDATING USER({})", id); 
        self.update_var(id, "user_name".to_string(), user.user_name).await?;
        self.update_var(id, "full_name".to_string(), user.full_name).await?;
        self.update_var(id, "email".to_string(), user.email).await?;
        self.update_var(id, "phone_number".to_string(), user.phone_number).await?;
        self.update_var(id, "alias".to_string(), user.alias.map(|e| e.to_string())).await?;
        Ok(())
    }

    async fn users(&self) -> Result<Vec<DBUser>> {
        println!("READING ALL USERS");
        let query_string = "SELECT * FROM user";
        let result = query_as(&query_string)
            .fetch_all(self.conn())
            .await
            .map_err(|e| {println!("Error: {}", e); Error::DatabaseError})?;

        Ok(result)
    }

    async fn user_by_id(&self, user_id: u64) -> Result<User> {
        println!("READING USER({})", user_id);
        let result = query_as("SELECT full_name, user_name, email_password, phone_number, alias FROM user WHERE id = $1")
            .bind(user_id.to_string())
            .fetch_one(self.conn())
            .await
            .map_err(|_| Error::DatabaseError)?;

        Ok(result)
    }

    async fn my_trustees(&self, user_id: u64) -> Result<Vec<DBUser>> {
        println!("READING USER({})", user_id);
        
        let result = query_as("SELECT * FROM user WHERE id IN (SELECT trustee FROM trust where trust.user = $1)")
            .bind(user_id.to_string())
            .fetch_all(self.conn())
            .await
            .map_err(|_| Error::DatabaseError)?;

        Ok(result)
    }

    async fn my_locations(&self, user_id: u64) -> Result<Vec<Location>> {
        println!("READING USER LOCASIONS({})", user_id);
        
        let result = query_as("SELECT * FROM location where user_id = $1")
            .bind(user_id.to_string())
            .fetch_all(self.conn())
            .await
            .map_err(|_| Error::DatabaseError)?;

        Ok(result)
    }



    async fn delete_user(&self, _user_id: u64) -> Result<()> {todo!()}
}

