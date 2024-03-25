use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct CryptoData {
    pub id: u64,
    pub name: String,
    pub price: f64,
    pub fee_amount: String,
    pub price_impact_pct: f64,
    pub slipage: String,
    pub date: String,
}

#[derive(Debug, FromRow)]
pub struct CreateCryptoData {
    pub name: String,
    pub price: f64,
    pub fee_amount: String,
    pub slipage: String,
    pub price_impact_pct: f64,
}
