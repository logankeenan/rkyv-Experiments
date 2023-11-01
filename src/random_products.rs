use chrono::Utc;
use fake::{Fake, faker, Faker};
use rand::prelude::SliceRandom;
use rand::Rng;
use strum::IntoEnumIterator;
use uuid::Uuid;
use crate::product::{ProductDimensions, Product, ProductCategory, ProductCurrency};

pub fn create_random_products() -> Vec<Product> {
    let mut rng = rand::thread_rng();
    let mut products: Vec<Product> = Vec::new();

    let categories: Vec<_> = ProductCategory::iter().collect();
    let currencies: Vec<_> = ProductCurrency::iter().collect();

    for _ in 0..1000 {
        let name: String = faker::company::en::Buzzword().fake();
        let brand: String = faker::company::en::CompanyName().fake();
        let sku = format!("SKU{}", rng.gen_range(10000..99999));
        let description: String = faker::lorem::en::Sentence(20..40).fake();
        let price: f64 = Faker.fake();
        let weight = rng.gen_range(1.0..10.0);
        let dimensions = ProductDimensions::new(rng.gen_range(1.0..10.0), rng.gen_range(1.0..10.0), rng.gen_range(1.0..10.0));
        let rating = rng.gen_range(1.0..5.0);
        let stock_quantity = rng.gen_range(1..100);
        let category = categories.choose(&mut rng).unwrap().clone();
        let manufacturer = faker::company::en::CompanyName().fake();
        let created_at = Utc::now();
        let updated_at = Utc::now();
        let currency = currencies.choose(&mut rng).unwrap().clone();

        products.push(Product {
            id: Uuid::new_v4(),
            name,
            brand,
            sku,
            description,
            price,
            weight,
            dimensions,
            rating,
            stock_quantity,
            category,
            created_at,
            updated_at,
            currency,
            manufacturer,
        });
    }

    products
}