use serde::{Deserialize, Serialize};
use log::info;

/// Product Bundle Manager
/// Create and manage product bundles
pub struct ProductBundles;

/// Bundle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bundle {
    pub id: String,
    pub name: String,
    pub description: String,
    pub products: Vec<BundleProduct>,
    pub bundle_price: f64,
    pub individual_price: f64,
    pub savings: f64,
    pub active: bool,
}

/// Product in bundle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleProduct {
    pub product_id: String,
    pub product_name: String,
    pub quantity: usize,
    pub price: f64,
}

impl ProductBundles {
    /// Create bundle
    pub async fn create_bundle(
        &self,
        name: String,
        description: String,
        products: Vec<BundleProduct>,
        bundle_price: f64,
    ) -> Result<Bundle, String> {
        if products.is_empty() {
            return Err("Bundle must contain at least one product".to_string());
        }

        let individual_price: f64 = products.iter()
            .map(|p| p.price * p.quantity as f64)
            .sum();

        let savings = individual_price - bundle_price;

        if savings < 0.0 {
            return Err("Bundle price cannot exceed individual prices".to_string());
        }

        info!("Creating bundle: {} (${:.2} savings)", name, savings);

        Ok(Bundle {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            products,
            bundle_price,
            individual_price,
            savings,
            active: true,
        })
    }

    /// Calculate bundle analytics
    pub async fn get_bundle_analytics(&self, bundle_id: &str) -> Result<BundleAnalytics, String> {
        info!("Getting analytics for bundle: {}", bundle_id);

        // TODO: Query actual sales data
        Ok(BundleAnalytics {
            bundle_id: bundle_id.to_string(),
            units_sold: 45,
            revenue: 4500.0,
            average_order_value: 100.0,
            conversion_rate: 12.5,
        })
    }
}

/// Bundle analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleAnalytics {
    pub bundle_id: String,
    pub units_sold: usize,
    pub revenue: f64,
    pub average_order_value: f64,
    pub conversion_rate: f64,
}

impl Default for ProductBundles {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_bundle() {
        let bundles = ProductBundles;
        let products = vec![
            BundleProduct {
                product_id: "prod-1".to_string(),
                product_name: "Product 1".to_string(),
                quantity: 1,
                price: 50.0,
            },
            BundleProduct {
                product_id: "prod-2".to_string(),
                product_name: "Product 2".to_string(),
                quantity: 1,
                price: 30.0,
            },
        ];

        let result = bundles.create_bundle(
            "Combo Deal".to_string(),
            "Great savings!".to_string(),
            products,
            70.0,
        ).await;

        assert!(result.is_ok());
        let bundle = result.unwrap();
        assert_eq!(bundle.savings, 10.0);
    }
}
