use crate::inventory::InventoryManager;
use crate::purchase::PurchaseManager;
use crate::sales::SalesManager;

pub struct ReportingManager;

#[allow(dead_code, unused_variables)]
impl ReportingManager {
    pub fn generate_inventory_report(inventory_manager: &InventoryManager) {
        println!("Inventory Report:");
        println!(
            "{:<20} {:<10} {:<10} {:<10}",
            "Product Name", "Quantity", "Price", "Total Value"
        );
        for product in inventory_manager.list_products() {
            let total_value = product.price * product.quantity as f64;
            println!(
                "{:<20} {:<10} ${:<9.2} ${:<9.2}",
                product.name, product.quantity, product.price, total_value
            );
        }
    }

    pub fn generate_sales_report(sales_manager: &SalesManager) {
        println!("\nSales Report:");
        println!(
            "{:<20} {:<10} {:<10} {:<10}",
            "Product Name", "Quantity Sold", "Sale Price", "Total Sales"
        );
        for sale in sales_manager.get_sales_history() {
            let total_sales = sale.sale_price * sale.quantity_sold as f64;
            println!(
                "{:<20} {:<10} ${:<9.2} ${:<9.2}",
                sale.product_name, sale.quantity_sold, sale.sale_price, total_sales
            );
        }
        println!("Total Sales: ${:.2}", sales_manager.get_total_sales());
        println!("Total Profit: ${:.2}", sales_manager.get_total_profit());
    }

    pub fn generate_purchase_report(purchase_manager: &PurchaseManager) {
        println!("\nPurchase Report:");
        println!(
            "{:<20} {:<15} {:<15} {:<15}",
            "Product Name", "Quantity Purchased", "Purchase Price", "Total Cost"
        );
        for purchase in purchase_manager.get_purchase_history() {
            let total_cost = purchase.purchase_price * purchase.quantity_purchased as f64;
            println!(
                "{:<20} {:<15} ${:<14.2} ${:<14.2}",
                purchase.product_name,
                purchase.quantity_purchased,
                purchase.purchase_price,
                total_cost
            );
        }
        println!(
            "Total Purchases: ${:.2}",
            purchase_manager.get_total_purchases()
        );
    }
}
