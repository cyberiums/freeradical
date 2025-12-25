# FreeRadical CMS - Phase 8 Commerce Architecture

## Extensible Payment System Design

The payment system uses a **trait-based plugin architecture** that allows adding new payment providers without modifying core code.

### Core Components

```
PaymentHandler (Trait)
    ├── StripePaymentHandler ✅ DEPLOYED
    ├── PayPalPaymentHandler ✅ DEPLOYED
    ├── SquarePaymentHandler ✅ DEPLOYED
    └── Custom handlers... (extensible)

PaymentHandlerRegistry
    └── Manages all registered handlers
```

### Adding a New Payment Provider

1. **Implement the `PaymentHandler` trait:**

```rust
use async_trait::async_trait;

pub struct PayPalPaymentHandler {
    client_id: String,
    client_secret: String,
}

#[async_trait]
impl PaymentHandler for PayPalPaymentHandler {
    fn provider_name(&self) -> &str {
        "paypal"
    }
    
    async fn create_payment_intent(...) -> Result<PaymentIntent, String> {
        // PayPal API implementation
    }
    
    // Implement other required methods...
}
```

2. **Register in `main.rs`:**

```rust
// Stripe
if let Ok(stripe_key) = std::env::var("STRIPE_SECRET_KEY") {
    payment_registry.register(Box::new(
        StripePaymentHandler::new(stripe_key)
    ));
}

// PayPal
if let (Ok(client_id), Ok(client_secret)) = (
    std::env::var("PAYPAL_CLIENT_ID"),
    std::env::var("PAYPAL_CLIENT_SECRET")
) {
    payment_registry.register(Box::new(
        PayPalPaymentHandler::new(client_id, client_secret, sandbox)
    ));
}

// Square
if let Ok(access_token) = std::env::var("SQUARE_ACCESS_TOKEN") {
    payment_registry.register(Box::new(
        SquarePaymentHandler::new(access_token, sandbox)
    ));
}
```

3. **Use via API:**

```bash
# Stripe Payment
POST /payments/create
{
  "amount_cents": 5000,
  "currency": "USD",
  "provider": "stripe",
  "metadata": {}
}

# PayPal Payment
POST /payments/create
{
  "amount_cents": 5000,
  "currency": "USD",
  "provider": "paypal",
  "metadata": {"order_id": "ORD-123"}
}

# Square Payment
POST /payments/create
{
  "amount_cents": 5000,
  "currency": "USD",
  "provider": "square",
  "metadata": {"source_id": "cnon:card-nonce-xyz"}
}

# Get Available Providers
GET /payments/providers
# Returns: {"providers": ["stripe", "paypal", "square"]}
```

### Benefits

- **Pluggable**: Add/remove providers at runtime via environment variables
- **Testable**: Mock payment handlers for testing
- **Consistent**: All providers implement same interface
- **Extensible**: Support any payment gateway (Stripe, PayPal, Square, Crypto, etc.)

### Database Schema

```
products → Core product catalog
orders → Customer orders with payment tracking
order_items → Line items linking products to orders
```

Each order tracks:
- `payment_provider`: Which handler was used
- `payment_intent_id`: Provider-specific payment ID
- `status`: Order state machine

This design ensures FreeRadical CMS can integrate with **any payment system** without architectural changes.
