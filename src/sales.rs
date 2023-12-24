//use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Sale {
    pub product_name: String,
    pub quantity_sold: u32,
    pub sale_price: f64,
}
#[allow(dead_code, unused_variables)]
impl Sale {
    pub fn new(product_name: &str, quantity_sold: u32, sale_price: f64) -> Self {
        Self {
            product_name: product_name.to_string(),
            quantity_sold,
            sale_price,
        }
    }
}

pub struct SalesManager {
    sales_history: Vec<Sale>,
    total_sales: f64,
    total_profit: f64,
}
#[allow(dead_code, unused_variables)]
impl SalesManager {
    pub fn new() -> Self {
        Self {
            sales_history: Vec::new(),
            total_sales: 0.0,
            total_profit: 0.0,
        }
    }

    pub fn record_sale(&mut self, sale: Sale, cost_price: f64) {
        self.total_sales += sale.sale_price * sale.quantity_sold as f64;
        self.total_profit += (sale.sale_price - cost_price) * sale.quantity_sold as f64;
        self.sales_history.push(sale);
    }

    pub fn get_total_sales(&self) -> f64 {
        self.total_sales
    }

    pub fn get_total_profit(&self) -> f64 {
        self.total_profit
    }

    pub fn get_sales_history(&self) -> &Vec<Sale> {
        &self.sales_history
    }
}
