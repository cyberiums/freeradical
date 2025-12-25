# FreeRadical CMS v1.0.0 - Integration Tests

This test suite validates the complete Commerce API functionality including Products and Orders.

## Test Coverage

- ✅ Product CRUD operations
- ✅ Order creation with line items
- ✅ Order-payment integration
- ✅ Authentication requirements
- ✅ Data validation

## Running Tests

```bash
cargo test --test commerce_integration_test
```

## Test Scenarios

### 1. Product Management
- Create product
- List products (pagination)
- Get single product
- Update product
- Soft delete product

### 2. Order Flow
- Create order with multiple items
- Validate product availability
- Calculate total correctly
- Link payment to order
- Update order status

### 3. Security
- Unauthenticated requests rejected
- Users can only access their own orders
- Admin-only operations protected
