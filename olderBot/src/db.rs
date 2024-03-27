// src/db.rs
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::model::{CreateCryptoData, CryptoData};

pub async fn establish_connection() -> sqlx::Pool<sqlx::Postgres> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}

pub async fn create_crypto_data(
    pool: &sqlx::Pool<sqlx::Postgres>,
    data: &CreateCryptoData,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    let rec = sqlx::query!(
        "INSERT INTO crypto_data (name, price, fee_amount, price_impact_pct, slipage)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING id",
        data.name,
        data.price,
        data.fee_amount,
        data.price_impact_pct,
        data.slipage
    )
    .fetch_one(&mut tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

// pub async fn get_users(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<User>, sqlx::Error> {
//     let mut conn = pool.acquire().await?;
//     let users = sqlx::query_as!(User, "SELECT id, name, email FROM users")
//         .fetch_all(&mut conn)
//         .await?;

//     Ok(users)
// }
