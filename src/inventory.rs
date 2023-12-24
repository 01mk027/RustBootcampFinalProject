use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
pub struct Product {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: u32,
}

impl Product {
    pub fn new(name: &str, description: &str, price: f64, quantity: u32) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            price,
            quantity,
        }
    }
}

pub struct InventoryManager {
    products: HashMap<String, Product>,
}

#[allow(dead_code, unused_variables)]
impl InventoryManager {
    pub fn new() -> Self {
        Self {
            products: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        self.products.insert(product.name.clone(), product);
    }

    pub fn edit_product(&mut self, name: &str, updated_product: Product) -> Result<(), &str> {
        
        if !self.products.contains_key(name) {
            return Err("Product not foundê");
        }
        self.products.insert(name.to_string(), updated_product);
        Ok(())
    }

    pub fn delete_product(&mut self, name: &str) -> Result<(), &str> {
        if self.products.remove(name).is_none() {
            return Err("Product not found");
        }
        Ok(())
    }

// Here to find product ??
    pub fn get_product(&self, name: &str) -> Option<&Product> {
        self.products.get(name)
    }

    pub fn list_products(&self) -> Vec<&Product> {
        self.products.values().collect()
    }

    pub fn get_product_data_from_user() -> Product {
        let mut product_name = String::new();
        println!("Ürün adını giriniz : ");
        io::stdin().read_line(&mut product_name).unwrap();
        let mut product_description = String::new();
        println!("Ürün tanımlanmasını giriniz : ");
        io::stdin().read_line(&mut product_description).unwrap();
        let mut product_price = String::new();
        println!("Ürün fiyatını giriniz : ");
        io::stdin().read_line(&mut product_price).unwrap();
        let product_price: f64 = product_price.trim().parse().unwrap();
        let mut product_quantity = String::new();
        println!("Ürün quantity giriniz : ");
        io::stdin().read_line(&mut product_quantity).unwrap();
        let product_quantity: u32 = product_quantity.trim().parse().unwrap();
        Product::new(
            product_name.as_str(),
            product_description.as_str(),
            product_price,
            product_quantity,
        )
    }
}
