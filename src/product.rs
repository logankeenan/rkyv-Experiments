use chrono::{DateTime, Utc};
use rkyv::{Archive, Serialize as RkyvSerialize};
use uuid::Uuid;
use strum_macros::EnumIter;
use serde::Serialize;

#[derive(EnumIter, Debug, Clone, Serialize, Archive, RkyvSerialize)]
pub enum ProductCategory {
    Electronics,
    Clothing,
    HomeGoods,
    Books,
    Toys,
}

#[derive(EnumIter, Debug, Clone, Serialize, Archive, RkyvSerialize)]
pub enum ProductCurrency {
    USD,
    GBP,
    EUR,
    JPY,
    AUD,
    CAD,
}

#[derive(Serialize, Clone, Archive, RkyvSerialize)]
pub struct ProductDimensions {
    length: f64,
    width: f64,
    height: f64,
}

impl ProductDimensions {
    pub(crate) fn new(length: f64, width: f64, height: f64) -> Self {
        Self { length, width, height }
    }
}


#[derive(Serialize, Clone, Archive, RkyvSerialize)]
pub struct Product {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) brand: String,
    pub(crate) sku: String,
    pub(crate) description: String,
    pub(crate) price: f64,
    pub(crate) weight: f64,
    pub(crate) dimensions: ProductDimensions,
    pub(crate) rating: f64,
    pub(crate) stock_quantity: u32,
    pub(crate) category: ProductCategory,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
    pub(crate) currency: ProductCurrency,
    pub(crate) manufacturer: String,
}