use crate::domain::entity::{
    discount_entity::{PaginatedResponse, PaginationMeta, ProductDiscount},
    product_entity::{CreateProduct, Product, UpdateProduct},
};
use crate::domain::repository::product_repository::ProductRepository;
use async_trait::async_trait;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct InMemoryProductRepository {
    products: Arc<RwLock<HashMap<String, Product>>>,
    discounts: Arc<RwLock<HashMap<String, ProductDiscount>>>,
}

impl InMemoryProductRepository {
    pub fn new() -> Self {
        Self {
            products: Arc::new(RwLock::new(HashMap::new())),
            discounts: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

unsafe impl Send for InMemoryProductRepository {}
unsafe impl Sync for InMemoryProductRepository {}

#[async_trait]
impl ProductRepository for InMemoryProductRepository {
    async fn find(&self, id: String) -> Result<Product, String> {
        let products = self.products.read().await;
        products
            .get(&id)
            .filter(|p| p.deleted_at.is_none())
            .cloned()
            .ok_or_else(|| "Product not found".to_string())
    }

    async fn find_all(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
        search: String,
        min_price: u32,
        max_price: u32,
        has_discount: bool,
    ) -> Result<PaginatedResponse<Product>, String> {
        let products = self.products.read().await;
        let discounts = self.discounts.read().await;

        let min_price = min_price as u64;
        let max_price = max_price as u64;
        let search = search.to_lowercase();

        let filtered_products: Vec<Product> = products
            .values()
            .filter(|p| p.deleted_at.is_none())
            .filter(|p| {
                if !search.is_empty() {
                    let name_match = p.name.to_lowercase().contains(&search);
                    let desc_match = p
                        .description
                        .as_ref()
                        .map(|d| d.to_lowercase().contains(&search))
                        .unwrap_or(false);
                    if !name_match && !desc_match {
                        return false;
                    }
                }

                if p.price < min_price || p.price > max_price {
                    return false;
                }

                if has_discount {
                    let has_valid_discount = discounts
                        .get(&p.id.to_string())
                        .map(|d| d.applied_at > Utc::now() - chrono::Duration::days(30))
                        .unwrap_or(false);
                    if !has_valid_discount {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect();

        let mut sorted_products = filtered_products;
        sorted_products.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        let page = page.unwrap_or(1);
        let limit = limit.unwrap_or(10);

        let total_items = sorted_products.len() as u64;
        let total_pages = (total_items as f64 / limit as f64).ceil() as u32;

        let start_index = ((page - 1) * limit) as usize;
        let end_index = std::cmp::min(start_index + limit as usize, sorted_products.len());

        let paginated_data = if start_index < sorted_products.len() {
            sorted_products[start_index..end_index].to_vec()
        } else {
            Vec::new()
        };

        Ok(PaginatedResponse {
            data: paginated_data,
            meta: PaginationMeta {
                page,
                limit,
                total_items,
                total_pages,
            },
        })
    }

    async fn create(&self, product: CreateProduct) -> Result<Product, String> {
        log::info!("Start creating product.");

        {
            let products = self.products.read().await;
            if products.values().any(|p| {
                p.deleted_at.is_none() && p.name.to_lowercase() == product.name.to_lowercase()
            }) {
                return Err("Product already exists".to_string());
            }
        }

        let mut products = self.products.write().await;
        let id = Uuid::new_v4();
        let now = Utc::now();

        let new_product = Product {
            id,
            name: product.name,
            stock: product.stock,
            description: product.description,
            price: product.price,
            created_at: now,
            updated_at: Some(now),
            deleted_at: None,
        };

        products.insert(id.to_string(), new_product.clone());

        log::info!("Product created.");
        Ok(new_product)
    }

    async fn update(&self, id: String, new_product: UpdateProduct) -> Result<Product, String> {
        let mut products = self.products.write().await;
        if let Some(product) = products.get_mut(&id) {
            if let Some(name) = new_product.name {
                product.name = name;
            }
            if let Some(description) = new_product.description {
                product.description = Some(description);
            }
            if let Some(price) = new_product.price {
                product.price = price;
            }
            if let Some(stock) = new_product.stock {
                product.stock = stock;
            }
            if let Some(_) = new_product.deleted_at {
                product.deleted_at = None;
            }
            product.updated_at = Some(Utc::now());
            Ok(product.clone())
        } else {
            Err("Product not found".to_string())
        }
    }

    async fn delete(&self, id: String) -> Result<(), String> {
        let mut products = self.products.write().await;
        if let Some(product) = products.get_mut(&id) {
            product.deleted_at = Some(Utc::now());
            Ok(())
        } else {
            Err("Product not found".to_string())
        }
    }

    async fn has_discount(&self, product_id: String) -> bool {
        let discounts = self.discounts.read().await;
        if let Some(discount) = discounts.get(&product_id) {
            discount.applied_at > Utc::now() - chrono::Duration::days(30)
        } else {
            false
        }
    }
}
