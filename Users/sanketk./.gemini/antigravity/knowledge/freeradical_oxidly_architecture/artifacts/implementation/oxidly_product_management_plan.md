# Product Management UI Implementation Plan (Finalized)

## Goal
Implement a complete Product Management interface (CRUD) in Oxidly, integrated with the FreeRadical Rust commerce backend.

## Technical Requirements
- **Backend Service**: Interacts with `/v1/products` endpoints.
- **ID Management**: Backend uses `i64` (integer) for primary keys but requires a `uuid` string in `NewProduct` for creation.
- **Currency Handling**: Prices are stored as `price_cents` (integer). UI must handle conversions to/from main currency units (e.g., dollars).
- **Data Model**: `name`, `description` (optional), `price_cents`, `sku` (optional), `inventory_count` (optional).

## Implementation Details

### API Service (`oxidly/services/api.js`)
Added the `products` namespace:
- `list(req)`: `GET /products`
- `get(id, req)`: `GET /products/:id`
- `create(data, req)`: `POST /products`
- `update(id, data, req)`: `PUT /products/:id`
- `delete(id, req)`: `DELETE /products/:id`

### Controller (`oxidly/controllers/product_controller.js`)
- **`index`**: Fetches product list. Backend returns `{ products: [], total: ... }`. Formats each `price_cents` into a decimal `price` string for display.
- **`create`**: 
    - Generates a `uuid` using `crypto.randomUUID()`.
    - Converts decimal `price` from the form to `price_cents` (Math.round(p * 100)).
- **`edit`**: Fetches product by integer ID. Formats price for the form.
- **`update`**: 
    - Takes integer ID from parameters.
    - Requires original `uuid` (passed via hidden field) as the `NewProduct` struct in backend expects it.
    - Performs unit conversion for price.
- **`delete`**: Calls the delete endpoint using integer ID.

### Views (`oxidly/views/products/`)
- **`index.hbs`**: Responsive table listing Name, SKU, Price, and Inventory. Includes status badges.
- **`form.hbs`**: Unified form for creation and editing.
    - Uses `req.params.id` or `product.id` to determine action.
    - Includes hidden `uuid` field for updates to satisfy backend model constraints.
    - Price input is `type="number"` with `step="0.01"`.

## Routing Table Updates
- `GET /products` -> `productController.index`
- `GET /products/new` -> `productController.new`
- `POST /products` -> `productController.create`
- `GET /products/:id/edit` -> `productController.edit`
- `POST /products/:id` -> `productController.update`
- `POST /products/:id/delete` -> `productController.delete`

## Verification Steps
1. Navigate to /products.
2. Click "New Product", fill details, and save.
3. Edit the product to change price/stock.
4. Delete the product.
