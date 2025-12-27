# Order Management UI Implementation Plan

## Goal
Implement a merchant-facing Order Management interface in Oxidly that enables tracking, status updates, and fulfillment oversight.

## Backend Analysis
Research into `src/controllers/order_controller.rs` and `src/models/commerce_models.rs` reveals the following architectural details:

### Data Structures
- **OrderResponse**: The primary GET endpoint for a single order returns a composite structure:
    - `order`: The core `Order` metadata (ID, status, total, payment status).
    - `items`: A collection of `OrderItemWithProduct` objects.
    - `total_amount_cents`: Redundant check for the order total.
- **OrderItemWithProduct**: Crucially includes `product_name`, `quantity`, `price_cents`, and `subtotal_amount_cents`.
    - *Note*: The backend manually resolves product names because schema joins are currently commented out.

### Verified Endpoints
- `GET /orders`: Returns a list of the current user's orders (descending by date).
- `GET /orders/{id}`: Returns full order details with product line items.
- `PUT /orders/{id}/status`: Updates the order status (restricted to: `pending`, `processing`, `completed`, `cancelled`).
- `PUT /orders/{id}/payment`: Links a payment provider and intent ID, transitioning the order to `processing`.

## Implementation Strategy

### 1. API Client Expansion (`services/api.js`)
Add the `orders` namespace:
- `list(req)`: `GET /orders`
- `get(id, req)`: `GET /orders/${id}`
- `updateStatus(id, status, req)`: `PUT /orders/${id}/status`

### 2. Order Controller (`controllers/order_controller.js`)
- **`index`**: Fetches and lists all orders for the merchant.
- **`show`**: Fetches a single order's details. Formats `price_cents` to decimal currency strings for the line items and total.
- **`updateStatus`**: Logic for processing fulfillment (e.g., moving an order from "Processing" to "Completed").

### 3. Views (`views/orders/`)
- **`index.hbs`**: A sortable table of orders showing Order #, Date, Customer (User UUID), Status (badges), and Total.
- **`details.hbs`**: A comprehensive receipt-style view displaying:
    - Order summary and status controls.
    - Customer information.
    - A line-item table (Product Name, Unit Price, Qty, Subtotal).
    - Payment status details.

## State Transitions
- **Pending**: Initial state upon creation.
- **Processing**: Set after payment is linked or manually updated.
- **Completed**: Set after fulfillment.
- **Cancelled**: Terminal state for failed or voided orders.

## Status
- Core CRUD: [x] Completed (2025-12-27).
- Formatting: [x] Currency (cents -> float) and Date localization implemented.
- Status Updates: [x] Persistence verified via backend `update_order_status` endpoint.
- Views: [x] Responsive list and detail receipts deployed.
