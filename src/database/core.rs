use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::result::Result;
use uuid::Uuid;
use crate::types::core_database::{User, Service};


pub struct CoreDatabase {
    pool: Pool<MySql>
}

pub async fn new_core_database() -> CoreDatabase {

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:Tr0nc8!-@35.204.5.144:3306/core").await.unwrap();

    CoreDatabase {
        pool
    }
}

impl CoreDatabase {

    pub async fn read_user_by_id(&self, user_id: &str) -> Result<User, sqlx::Error> {
        let result = sqlx::query_as::<_,User>("SELECT * FROM user WHERE id = ?")
            .bind(user_id)
            .fetch_one(&self.pool).await?;
        Ok(result)
    }

    pub async fn create_user(&self, name: &String, email: &String, password: &String, last_login: &String, verified: bool) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        match sqlx::query("INSERT INTO user VALUES (?, ?, ?, ?, ?, ?)")
            .bind(Uuid::new_v4().to_string())
            .bind(name)
            .bind(email)
            .bind(password)
            .bind(last_login)
            .bind(verified)
            .execute(&mut *tx).await {
                Ok(_) => {
                    tx.commit().await?;
                    Ok(())
                },
                Err(e) => {
                    tx.rollback().await?;
                    Err(e)
                }
            }
    }

    pub async fn read_services(&self) -> Result<Vec<Service>, sqlx::Error> {
        let result = sqlx::query_as::<_, Service>("SELECT * FROM service ORDER BY name").fetch_all(&self.pool).await?;
        Ok(result)
    }

}

