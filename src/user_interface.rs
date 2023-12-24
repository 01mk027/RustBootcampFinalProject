use crate::inventory::InventoryManager;
use crate::purchase::PurchaseManager;
use crate::reporting::ReportingManager;
use crate::sales::SalesManager;
use crate::security::*;
use std::io::{self, Write};

use crate::data_input_from_user::*;

pub struct UserInterface;

#[allow(dead_code, unused_variables)]
impl UserInterface {
    pub fn start() {
        let mut security_manager = SecurityManager::new();
        let user = security_manager.login("admin", "password").unwrap();

        let mut inventory_manager = InventoryManager::new();
        let sales_manager = SalesManager::new();
        let purchase_manager = PurchaseManager::new();
        loop {
            println!("Rusty Store Inventory Management System");
            println!("1. Manage Inventory");
            println!("2. Record a Sale");
            println!("3. Record a Purchase");
            println!("4. Generate Reports");
            println!("5. Exit");
            print!("Choose an option: ");
            io::stdout().flush().unwrap();

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();
            let choice = choice.trim();

            match choice {
                "1" => Self::manage_inventory(&mut inventory_manager, &security_manager),
                "2" => Self::record_sale(&sales_manager),
                "3" => Self::record_purchase(&purchase_manager),
                "4" => {
                    Self::generate_reports(&inventory_manager, &sales_manager, &purchase_manager)
                }
                "5" => break,
                _ => println!("Invalid choice, please choose a number between 1 and 5"),
            }
        }
    }

    fn manage_inventory(
        inventory_manager: &mut InventoryManager,
        security_manager: &SecurityManager,
    ) {
        // Implementation to manage inventory
        // You can list products, add a new product, update a product, or delete a product
        loop {
            println!("1 - Add product");
            println!("2 - Update product");
            println!("3 - List product");
            println!("4 - Delete product");
            println!("5 - Back to the main menu");
            print!("Choose an option: ");
            io::stdout().flush().unwrap();

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();
            let choice = choice.trim();

            match choice {
                "1" => {
                    //let product = Product::new("Product 1", "A sample product", 9.99, 100);
                    let product = get_product_data_from_user(&security_manager);
                    inventory_manager.add_product(product);
                    ReportingManager::generate_inventory_report(&inventory_manager);
                }
                "2" => {
                    let mut product_name = String::new();
                    println!("enter name of the product which you want to find...");
                    io::stdin().read_line(&mut product_name).unwrap();
                    match inventory_manager.get_product(&product_name)
                    {
                        Some(mut value) => {
                            //update from there...
                            //println!("enter name in get_product:");    
                            //let mut new_name = String::new();
                            //io::stdin().read_line(&mut new_name).unwrap();
                            
                            println!("{:?}", value);
                          
                            /*
                            println!("enter new name of the product:");
                            let mut new_name = String::new();
                            io::stdin().read_line(&mut new_name).unwrap();
                            */
                        
                            //let mut new_name = String::new();
                            //io::stdin().read_line(&mut new_name).unwrap();
                            //let product = inventory_manager.get_product(&product_name);
                            //println!("{:?}", product);
                            //let mut new_name = String::new();
                            //println!("Please enter new name of the product:");
                            //io::stdin().read_line(&mut new_name).unwrap();
                            //println!("{:?} {}", product, new_name);
                            
                            //io::stdin().read_line(&mut new_name).unwrap();
                            //match inventory_manager.edit_product()
                            //let mut new_name = String::new();
                            //inventory_manager.edit_product(&new_name, &product);
                            //io::stdin().read_line(&mut new_name).unwrap();
                        },
                        None => println!("product not found!")
                    }
                }
                "3" => {
                    let all_products = inventory_manager.list_products();
                    println!("List of products: \n");
                    println!("{:?}", all_products);
                }
                "4" => {
                    let mut product_name = String::new();
                    println!("Please enter the name of the product: \n");
                    io::stdin().read_line(&mut product_name).unwrap();
                    //let trimmed_product_name = product_name.trim();
                    match inventory_manager.delete_product(&product_name){
                        Ok(()) => {
                            inventory_manager.list_products().retain(|&x| *x.name == product_name);
                            println!("{} is removed successfully", product_name);
                        },
                        Err(e) => println!("{:?}", e)
                    }
                }
                "5" => break,
                _ => println!("Invalid choice, please choose a number between 1 and 5"),
            }
        }
    }

    fn record_sale(sales_manager: &SalesManager) {
        // Implementation to record a sale
        // You can enter product details, quantity sold, and sale price
    }

    fn record_purchase(purchase_manager: &PurchaseManager) {
        // Implementation to record a purchase
        // You can enter product details, quantity purchased, and purchase price
    }

    fn generate_reports(
        inventory_manager: &InventoryManager,
        sales_manager: &SalesManager,
        purchase_manager: &PurchaseManager,
    ) {
        ReportingManager::generate_inventory_report(inventory_manager);
        ReportingManager::generate_sales_report(sales_manager);
        ReportingManager::generate_purchase_report(purchase_manager);
    }
}
