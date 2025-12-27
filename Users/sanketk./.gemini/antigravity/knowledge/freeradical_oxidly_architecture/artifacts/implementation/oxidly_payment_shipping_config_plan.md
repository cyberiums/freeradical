# Payment & Shipping Configuration UI Implementation Plan

## Goal
Implement the Payment and Shipping configuration interface in Oxidly. This allows merchants to see active payment handlers (configured in the backend) and set basic shipping preferences.

## Technical Details
- **Backend API**: Interacts with `GET /v1/payments/providers` to list enabled handlers (e.g., Stripe, PayPal).
- **Settings Store**: Shipping settings (Flat Rate, Free Shipping Threshold) are currently handled as a simulated post to the `StoreController` without a dedicated backend settings model (aligned with ECRSS strategy).

## Implementation Details

### API Service (`oxidly/services/api.js`)
Added the `payments` namespace:
- `providers(req)`: `GET /payments/providers`

### Controller (`oxidly/controllers/store_controller.js`)
- **`settings`**: Fetches the list of providers from the backend and renders the settings view.
- **`updateSettings`**: Processes the shipping configuration form submission (simulated save).

### Views (`oxidly/views/store/ settings.hbs`)
- **Payment Providers**: Displays a list of active backend handlers with status badges. Includes a note that additional providers are configured via environment variables.
- **Shipping Configuration**: Provides inputs for Flat Rate and Free Shipping Threshold.

## Routing
- `GET /store/settings` -> `storeController.settings`
- `POST /store/settings` -> `storeController.updateSettings`

## Verification
1. Navigate to /store/settings.
2. Verify the "Payment Providers" list reflects the enabled backend handlers.
3. Submit the Shipping Configuration form and verify the success message.
