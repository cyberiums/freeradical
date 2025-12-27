# Oxidly Commerce System Implementation Guide

This guide consolidates the implementation of commerce-related features in the Oxidly Cloud Platform, integrating with the FreeRadical Rust backend.

## 1. Store Setup Wizard
Guides merchants through the initial configuration of their store identities and preferences.

- **Flow**: Multi-step single-page interface with Client-side navigation (Next/Back).
- **Steps**:
    1. **General Details**: Name, Industry, Contact Email.
    2. **Location**: Physical address data.
    3. **Preferences**: Currency selection (e.g., USD, EUR) and measurement units.
- **Persistence**: Simulated persistence in `StoreController` (aligned with ECRSS strategy).

## 2. Product Management (CRUD)
Complete lifecycle management for products with integer-based price handling.

- **Backend Integration**: Routes mapped to `/v1/products`.
- **Key Concepts**:
    - **ID Management**: Uses integer IDs for updates/deletes, but requires a UUID (generated client-side via `crypto.randomUUID()`) for creation.
    - **Price Handling**: Prices are stored as `price_cents` (integer). The UI converts these to/from decimal units (e.g., `value * 100`).
- **Views**: Responsive table for listing and a unified form for Creation/Editing.

## 3. Order Management
Merchant interface for tracking sales and managing fulfillment.

- **Data Model**: `OrderResponse` includes the order metadata and a list of `OrderItemWithProduct` (which resolves product names automatically in the backend).
- **Core Endpoints**:
    - `GET /orders`: List orders by date.
    - `GET /orders/{id}`: Detailed receipt-style view.
    - `PUT /orders/{id}/status`: Transition between `pending`, `processing`, `completed`, and `cancelled`.
- **Views**: Sortable order list and detailed fulfillment interface.

## 4. Payment & Shipping Configuration
Configuration for payment acceptance and delivery rules.

- **Payment Providers**: Dynamically fetched from `/v1/payments/providers` (lists enabled handlers like Stripe, PayPal, Square).
- **Shipping Settings**: Simulated configuration for Flat Rate and Free Shipping Thresholds.

---
*Note: This document consolidated and superseded `oxidly_product_management_plan.md`, `oxidly_order_management_plan.md`, `oxidly_payment_shipping_config_plan.md`, and `oxidly_store_setup_wizard_plan.md`.*
