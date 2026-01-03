use serde::{Deserialize, Serialize};
use log::info;

/// Inventory Analytics Engine
/// Advanced analytics for inventory performance
pub struct InventoryAnalytics;

/// Analytics report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryReport {
    pub total_products: usize,
    pub total_value: f64,
    pub turnover_rate: f64,
    pub best_sellers: Vec<ProductPerformance>,
    pub worst_sellers: Vec<ProductPerformance>,
    pub low_stock_items: Vec<StockAlert>,
    pub recommendations: Vec<String>,
}

/// Product performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductPerformance {
    pub product_id: String,
    pub product_name: String,
    pub units_sold: usize,
    pub revenue: f64,
    pub profit_margin: f64,
    pub velocity: f64, // Units per day
}

/// Stock alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockAlert {
    pub product_id: String,
    pub product_name: String,
    pub current_stock: usize,
    pub reorder_point: usize,
    pub days_until_stockout: Option<usize>,
}

impl InventoryAnalytics {
    /// Generate comprehensive inventory report
    pub async fn generate_report(&self, days: u32) -> Result<InventoryReport, String> {
        info!("Generating inventory analytics report for {} days", days);

        let products = self.get_product_data().await?;
        let total_value = self.calculate_total_value(&products);
        let turnover = self.calculate_turnover_rate(&products, days);
        let best = self.find_best_sellers(&products, 5);
        let worst = self.find_worst_sellers(&products, 5);
        let low_stock = self.find_low_stock_items(&products);
        let recommendations = self.generate_recommendations(&products, turnover);

        Ok(InventoryReport {
            total_products: products.len(),
            total_value,
            turnover_rate: turnover,
            best_sellers: best,
            worst_sellers: worst,
            low_stock_items: low_stock,
            recommendations,
        })
    }

    /// Get product data (mock)
    async fn get_product_data(&self) -> Result<Vec<ProductData>, String> {
        // TODO: Query actual database
        Ok(vec![
            ProductData {
                id: "prod-1".to_string(),
                name: "Premium Widget".to_string(),
                stock: 50,
                cost: 10.0,
                price: 25.0,
                units_sold_period: 120,
                reorder_point: 20,
            },
            ProductData {
                id: "prod-2".to_string(),
                name: "Standard Gadget".to_string(),
                stock: 5,
                cost: 5.0,
                price: 12.0,
                units_sold_period: 30,
                reorder_point: 15,
            },
            ProductData {
                id: "prod-3".to_string(),
                name: "Luxury Item".to_string(),
                stock: 100,
                cost: 50.0,
                price: 150.0,
                units_sold_period: 10,
                reorder_point: 25,
            },
        ])
    }

    /// Calculate total inventory value
    fn calculate_total_value(&self, products: &[ProductData]) -> f64 {
        products.iter()
            .map(|p| (p.stock as f64) * p.cost)
            .sum()
    }

    /// Calculate inventory turnover rate
    fn calculate_turnover_rate(&self, products: &[ProductData], days: u32) -> f64 {
        let total_sold: usize = products.iter().map(|p| p.units_sold_period).sum();
        let avg_inventory: f64 = products.iter().map(|p| p.stock as f64).sum::<f64>() / products.len() as f64;

        if avg_inventory == 0.0 {
            return 0.0;
        }

        (total_sold as f64 / avg_inventory) * (365.0 / days as f64)
    }

    /// Find best selling products
    fn find_best_sellers(&self, products: &[ProductData], top_n: usize) -> Vec<ProductPerformance> {
        let mut performances: Vec<ProductPerformance> = products.iter()
            .map(|p| self.calculate_performance(p))
            .collect();

        performances.sort_by(|a, b| b.units_sold.cmp(&a.units_sold));
        performances.truncate(top_n);
        performances
    }

    /// Find worst selling products
    fn find_worst_sellers(&self, products: &[ProductData], bottom_n: usize) -> Vec<ProductPerformance> {
        let mut performances: Vec<ProductPerformance> = products.iter()
            .map(|p| self.calculate_performance(p))
            .collect();

        performances.sort_by(|a, b| a.units_sold.cmp(&b.units_sold));
        performances.truncate(bottom_n);
        performances
    }

    /// Calculate product performance
    fn calculate_performance(&self, product: &ProductData) -> ProductPerformance {
        let revenue = (product.units_sold_period as f64) * product.price;
        let cost = (product.units_sold_period as f64) * product.cost;
        let profit_margin = if revenue > 0.0 {
            ((revenue - cost) / revenue) * 100.0
        } else {
            0.0
        };

        ProductPerformance {
            product_id: product.id.clone(),
            product_name: product.name.clone(),
            units_sold: product.units_sold_period,
            revenue,
            profit_margin,
            velocity: product.units_sold_period as f64 / 30.0, // Per day
        }
    }

    /// Find low stock items
    fn find_low_stock_items(&self, products: &[ProductData]) -> Vec<StockAlert> {
        products.iter()
            .filter(|p| p.stock <= p.reorder_point)
            .map(|p| {
                let days_until_stockout = if p.units_sold_period > 0 {
                    Some((p.stock as f64 / (p.units_sold_period as f64 / 30.0)).ceil() as usize)
                } else {
                    None
                };

                StockAlert {
                    product_id: p.id.clone(),
                    product_name: p.name.clone(),
                    current_stock: p.stock,
                    reorder_point: p.reorder_point,
                    days_until_stockout,
                }
            })
            .collect()
    }

    /// Generate recommendations
    fn generate_recommendations(&self, products: &[ProductData], turnover: f64) -> Vec<String> {
        let mut recommendations = vec![];

        if turnover < 4.0 {
            recommendations.push("Low inventory turnover - consider promotions or product mix changes".to_string());
        } else if turnover > 12.0 {
            recommendations.push("High turnover - ensure adequate stock levels to avoid stockouts".to_string());
        }

        let low_stock_count = products.iter().filter(|p| p.stock <= p.reorder_point).count();
        if low_stock_count > 0 {
            recommendations.push(format!("Reorder {} low-stock items immediately", low_stock_count));
        }

        let slow_movers = products.iter().filter(|p| p.units_sold_period < 10).count();
        if slow_movers > products.len() / 3 {
            recommendations.push("Many slow-moving items - review pricing or marketing strategy".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push("Inventory health looks good!".to_string());
        }

        recommendations
    }
}

/// Product data structure
#[derive(Debug, Clone)]
struct ProductData {
    id: String,
    name: String,
    stock: usize,
    cost: f64,
    price: f64,
    units_sold_period: usize,
    reorder_point: usize,
}

impl Default for InventoryAnalytics {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_report() {
        let analytics = InventoryAnalytics;
        let result = analytics.generate_report(30).await;

        assert!(result.is_ok());
        let report = result.unwrap();
        assert!(report.total_products > 0);
        assert!(report.total_value > 0.0);
    }

    #[test]
    fn test_calculate_total_value() {
        let analytics = InventoryAnalytics;
        let products = vec![
            ProductData {
                id: "1".to_string(),
                name: "Test".to_string(),
                stock: 10,
                cost: 5.0,
                price: 10.0,
                units_sold_period: 0,
                reorder_point: 5,
            },
        ];

        let value = analytics.calculate_total_value(&products);
        assert_eq!(value, 50.0);
    }
}
