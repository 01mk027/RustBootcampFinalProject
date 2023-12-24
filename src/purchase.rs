//use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Purchase {
    pub product_name: String,
    pub quantity_purchased: u32,
    pub purchase_price: f64,
}
#[allow(dead_code, unused_variables)]
impl Purchase {
    pub fn new(product_name: &str, quantity_purchased: u32, purchase_price: f64) -> Self {
        Self {
            product_name: product_name.to_string(),
            quantity_purchased,
            purchase_price,
        }
    }
}

pub struct PurchaseManager {
    purchase_history: Vec<Purchase>,
    total_purchases: f64,
}

#[allow(dead_code, unused_variables)]
impl PurchaseManager {
    pub fn new() -> Self {
        Self {
            purchase_history: Vec::new(),
            total_purchases: 0.0,
        }
    }

    pub fn record_purchase(&mut self, purchase: Purchase) {
        self.total_purchases += purchase.purchase_price * purchase.quantity_purchased as f64;
        self.purchase_history.push(purchase);
    }

    pub fn get_total_purchases(&self) -> f64 {
        self.total_purchases
    }

    pub fn get_purchase_history(&self) -> &Vec<Purchase> {
        &self.purchase_history
    }
}
