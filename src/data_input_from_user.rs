use crate::{
    inventory::*,
    security::{SecurityManager, UserRole},
};
use std::io;

pub fn get_product_data_from_user(security_manager: &SecurityManager) -> Product {
    if security_manager.check_permission(UserRole::Admin) {
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
    } else {
        panic!("Bu işlemi gerçekleştirmek için yetkiniz yok.")
    }
}
