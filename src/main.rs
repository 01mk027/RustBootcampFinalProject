mod data_input_from_user;
mod inventory;
mod purchase;
mod reporting;
mod sales;
mod security;
mod user_interface;

//use crate::inventory::{InventoryManager, Product};
//use crate::purchase::{Purchase, PurchaseManager};
//use crate::reporting::ReportingManager;
//use crate::sales::{Sale, SalesManager};
//use crate::security::SecurityManager;
//use crate::security::User;
use crate::user_interface::UserInterface;

//use std::io;

#[allow(dead_code, unused_variables)]
fn main() {
    // let mut security_manager = SecurityManager::new();
    // let user_login = security_manager.login("admin", "password");
    // let user = match security_manager.current_user {
    //     Some(user) => user,
    //     None => panic!(""),
    // };

    // let mut inventory_manager = InventoryManager::new();

    // let product = Product::new("Product 1", "A sample product", 9.99, 100);
    // let product1 = Product::new("Product 1", "A sample product", 9.99, 100);
    // //let product2 = get_product_data_from_user();
    // inventory_manager.add_product(product, &user);
    // inventory_manager.add_product(product1, &user);

    // // SALES
    // // let mut sales_manager = SalesManager::new();

    // // let sale = Sale::new("Product 1", 5, 9.99);
    // // sales_manager.record_sale(sale, 5.0);
    // // println!("Hello, world!");

    // // // PURCHASE
    // // let mut purchase_manager = PurchaseManager::new();

    // // let purchase = Purchase::new("Product 1", 5, 7.99);
    // // purchase_manager.record_purchase(purchase);

    // // // REPORTING
    // ReportingManager::generate_inventory_report(&inventory_manager);
    // ReportingManager::generate_sales_report(&sales_manager);
    // ReportingManager::generate_purchase_report(&purchase_manager);

    //
    UserInterface::start();
}
