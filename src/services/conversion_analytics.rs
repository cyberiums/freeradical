use serde::{Deserialize, Serialize};
use log::info;

/// Conversion Analytics
/// Track and analyze conversion funnel
pub struct ConversionAnalytics;

/// Conversion funnel data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionFunnel {
    pub period_days: u32,
    pub visitors: usize,
    pub cart_additions: usize,
    pub checkouts_started: usize,
    pub orders_completed: usize,
    pub conversion_rate: f64,
    pub drop_off_rates: DropOffRates,
}

/// Drop-off rates at each stage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropOffRates {
    pub browse_to_cart: f64,
    pub cart_to_checkout: f64,
    pub checkout_to_order: f64,
}

impl ConversionAnalytics {
    /// Get conversion funnel data
    pub async fn get_funnel(&self, days: u32) -> Result<ConversionFunnel, String> {
        info!("Getting conversion funnel for {} days", days);

        // TODO: Query actual analytics data
        let visitors = 10000;
        let cart_additions = 2500;
        let checkouts_started = 1500;
        let orders_completed = 750;

        let conversion_rate = (orders_completed as f64 / visitors as f64) * 100.0;

        let drop_off_rates = DropOffRates {
            browse_to_cart: ((visitors - cart_additions) as f64 / visitors as f64) * 100.0,
            cart_to_checkout: ((cart_additions - checkouts_started) as f64 / cart_additions as f64) * 100.0,
            checkout_to_order: ((checkouts_started - orders_completed) as f64 / checkouts_started as f64) * 100.0,
        };

        Ok(ConversionFunnel {
            period_days: days,
            visitors,
            cart_additions,
            checkouts_started,
            orders_completed,
            conversion_rate,
            drop_off_rates,
        })
    }

    /// Calculate revenue attribution
    pub async fn calculate_attribution(&self) -> Result<RevenueAttribution, String> {
        info!("Calculating revenue attribution");

        Ok(RevenueAttribution {
            direct: 45.0,
            organic_search: 25.0,
            paid_search: 15.0,
            social: 10.0,
            email: 5.0,
        })
    }
}

/// Revenue attribution by channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueAttribution {
    pub direct: f64,
    pub organic_search: f64,
    pub paid_search: f64,
    pub social: f64,
    pub email: f64,
}

impl Default for ConversionAnalytics {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_funnel() {
        let analytics = ConversionAnalytics;
        let result = analytics.get_funnel(30).await;

        assert!(result.is_ok());
        let funnel = result.unwrap();
        assert!(funnel.conversion_rate > 0.0);
    }
}
