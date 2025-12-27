use serde::{Deserialize, Serialize};
use log::info;
use chrono::{DateTime, Utc, Duration};

/// Cart Abandonment Tracker
/// Track and recover abandoned carts
pub struct CartAbandonment;

/// Abandoned cart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbandonedCart {
    pub cart_id: String,
    pub user_id: Option<String>,
    pub email: Option<String>,
    pub items: Vec<CartItem>,
    pub total_value: f64,
    pub abandoned_at: DateTime<Utc>,
    pub recovery_sent: bool,
    pub recovered: bool,
}

/// Cart item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItem {
    pub product_id: String,
    pub product_name: String,
    pub quantity: usize,
    pub price: f64,
}

/// Recovery campaign
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryCampaign {
    pub total_abandoned: usize,
    pub total_value: f64,
    pub recovery_rate: f64,
    pub recovered_value: f64,
    pub emails_sent: usize,
}

impl CartAbandonment {
    /// Track abandoned cart
    pub async fn track_abandonment(
        &self,
        cart_id: String,
        user_id: Option<String>,
        email: Option<String>,
        items: Vec<CartItem>,
    ) -> Result<AbandonedCart, String> {
        if items.is_empty() {
            return Err("Cart cannot be empty".to_string());
        }

        let total_value = items.iter().map(|item| item.price * item.quantity as f64).sum();

        info!("Tracking cart abandonment: {} (value: ${:.2})", cart_id, total_value);

        Ok(AbandonedCart {
            cart_id,
            user_id,
            email,
            items,
            total_value,
            abandoned_at: Utc::now(),
            recovery_sent: false,
            recovered: false,
        })
    }

    /// Send recovery email
    pub async fn send_recovery_email(&self, cart: &AbandonedCart) -> Result<(), String> {
        if cart.email.is_none() {
            return Err("No email address available".to_string());
        }

        info!("Sending recovery email for cart: {}", cart.cart_id);

        // TODO: Integrate with email service
        Ok(())
    }

    /// Get recovery campaign stats
    pub async fn get_campaign_stats(&self, days: u32) -> Result<RecoveryCampaign, String> {
        info!("Getting recovery campaign stats for {} days", days);

        // TODO: Query actual database
        let mock_data = self.get_mock_stats();

        Ok(mock_data)
    }

    /// Mock stats for testing
    fn get_mock_stats(&self) -> RecoveryCampaign {
        RecoveryCampaign {
            total_abandoned: 150,
            total_value: 12500.0,
            recovery_rate: 15.5,
            recovered_value: 1937.5,
            emails_sent: 120,
        }
    }

    /// Identify high-value abandonments
    pub async fn find_high_value_carts(&self, min_value: f64) -> Result<Vec<AbandonedCart>, String> {
        info!("Finding high-value abandoned carts (min: ${:.2})", min_value);

        // TODO: Query database with filter
        let all_carts = self.get_mock_carts();
        let high_value: Vec<AbandonedCart> = all_carts.into_iter()
            .filter(|cart| cart.total_value >= min_value)
            .collect();

        Ok(high_value)
    }

    /// Mock carts for testing
    fn get_mock_carts(&self) -> Vec<AbandonedCart> {
        vec![
            AbandonedCart {
                cart_id: "cart-1".to_string(),
                user_id: Some("user-1".to_string()),
                email: Some("user1@example.com".to_string()),
                items: vec![
                    CartItem {
                        product_id: "prod-1".to_string(),
                        product_name: "Premium Widget".to_string(),
                        quantity: 2,
                        price: 50.0,
                    },
                ],
                total_value: 100.0,
                abandoned_at: Utc::now() - Duration::hours(2),
                recovery_sent: false,
                recovered: false,
            },
        ]
    }

    /// Calculate recovery potential
    pub fn calculate_recovery_potential(&self, cart: &AbandonedCart) -> f64 {
        let hours_since_abandonment = (Utc::now() - cart.abandoned_at).num_hours() as f64;

        // Recovery rate decreases over time
        let base_rate = if hours_since_abandonment < 1.0 {
            0.35 // 35% within 1 hour
        } else if hours_since_abandonment < 24.0 {
            0.25 // 25% within 24 hours
        } else if hours_since_abandonment < 72.0 {
            0.15 // 15% within 3 days
        } else {
            0.05 // 5% after 3 days
        };

        // High-value carts have better recovery
        let value_multiplier = if cart.total_value > 100.0 { 1.2 } else { 1.0 };

        base_rate * value_multiplier
    }
}

impl Default for CartAbandonment {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_track_abandonment() {
        let tracker = CartAbandonment;
        let items = vec![
            CartItem {
                product_id: "prod-1".to_string(),
                product_name: "Test Product".to_string(),
                quantity: 1,
                price: 25.0,
            },
        ];

        let result = tracker.track_abandonment(
            "cart-123".to_string(),
            Some("user-1".to_string()),
            Some("test@example.com".to_string()),
            items,
        ).await;

        assert!(result.is_ok());
        let cart = result.unwrap();
        assert_eq!(cart.total_value, 25.0);
    }

    #[tokio::test]
    async fn test_empty_cart() {
        let tracker = CartAbandonment;
        let result = tracker.track_abandonment(
            "cart-123".to_string(),
            None,
            None,
            vec![],
        ).await;

        assert!(result.is_err());
    }

    #[test]
    fn test_recovery_potential() {
        let tracker = CartAbandonment;
        let cart = AbandonedCart {
            cart_id: "test".to_string(),
            user_id: None,
            email: None,
            items: vec![],
            total_value: 150.0,
            abandoned_at: Utc::now() - Duration::hours(2),
            recovery_sent: false,
            recovered: false,
        };

        let potential = tracker.calculate_recovery_potential(&cart);
        assert!(potential > 0.0);
        assert!(potential <= 1.0);
    }
}
