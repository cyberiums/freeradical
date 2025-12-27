# Pluggable Payment Architecture

The FreeRadical e-commerce system uses a trait-based plugin architecture for extensibility.

## Trait-Based Design
All payment providers must implement the `PaymentHandler` trait (Rust/async-trait). This ensures a consistent interface across different gateways.

### Implementation Pattern
```rust
#[async_trait]
pub trait PaymentHandler {
    fn provider_name(&self) -> &str;
    async fn create_payment_intent(...) -> Result<PaymentIntent, String>;
    // ...other lifecycle methods
}
```

## Supported Providers (v1.7.0)
- **Stripe**: Deployed and production-ready.
- **PayPal**: Deployed and production-ready.
- **Square**: Deployed and production-ready.

## Configuration
Providers are registered at runtime via environment variables (e.g., `STRIPE_SECRET_KEY`, `PAYPAL_CLIENT_ID`). This allows for dynamic activation/deactivation of payment methods without code changes.

## Data Model
- **Orders**: Tracks `payment_provider` and `payment_intent_id`.
- **Status Machine**: Manages transitions (Pending → Paid → Failed) consistently regardless of the provider.
