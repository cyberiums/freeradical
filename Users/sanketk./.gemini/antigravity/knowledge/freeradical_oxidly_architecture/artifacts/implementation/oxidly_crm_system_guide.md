# Oxidly CRM Frontend Implementation Guide

The Oxidly CRM interface provides a user-friendly management system for customer data, interactions, and marketing campaigns, acting as a BFF (Backend-for-Frontend) layer over the FreeRadical Rust CRM services.

## 1. System Capabilities & Requirements

### Customer Profiles
Each customer record supports detailed fields to enable hyper-personalization:
- **Basic Info**: Name, Email (Primary Key), Phone, Year of Birth.
- **Location**: Country, City, Address, Timezone.
- **Status**: Active, Inactive, Suspended, Banned.
- **Relational Data**: Orders (linked via `customer_uuid`), Product History, Reviews/Ratings, and Interaction history.

### Analytics & AI Insights
- **Engagement Scoring**: Calculated based on order frequency and review activity.
- **Sentiment Analysis**: AI-powered analysis of customer reviews and support interactions.
- **Predictive Analytics**: Forecasting future purchase behavior (CLV - Customer Lifetime Value).
- **AI Customer Insights**: Automated grouping of customers into personas based on behavior.

### AI-Enabled CRM Features
- **AI Automated Messaging**: Natural language generation for personalized email/SMS responses.
- **Semantic Search**: Searching for customers using natural language queries.

## 2. Architecture

The CRM implementation follows the standard Oxidly BFF pattern:
1.  **API Client**: Methods in `oxidly/services/api.js` to communicate with the Rust backend.
2.  **Controller**: `oxidly/controllers/crm_controller.js` manages logic, data fetching, and rendering.
3.  **Views**: Handlebars templates in `oxidly/views/crm/` for the user interface.
4.  **Routes**: Registered in `oxidly/server.js`.

## 3. API Client Expansion (`api.js`)

The `api.js` service was expanded with a `crm` object supporting:
- `customers.list(filters)`: GET `/api/crm/customers`
- `customers.get(id)`: GET `/api/crm/customers/{id}` (Returns details including interactions and tasks)
- `customers.create(data)`: POST `/api/crm/customers` (Requires a `user_id` to link/create)
- `customers.update(id, data)`: PUT `/api/crm/customers/{id}`
- `customers.delete(id)`: DELETE `/api/crm/customers/{id}`
- `customers.getNotes(id)`: GET `/api/crm/customers/{id}/notes`
- `customers.addNote(id, data)`: POST `/api/crm/customers/{id}/notes`

## 4. Controller Logic (`crm_controller.js`)

The `CrmController` handles the following actions:
- **`index`**: Lists customers with pagination and filtering by `lifecycle_stage` and `churn_risk`.
- **`show`**: Displays a comprehensive customer dashboard. It combines the `CustomerDetailResponse` (profile, interactions, tasks) with a separate call to fetch `notes`.
- **`new / create`**: Handles adding new customers. *Note: Currently requires a valid User ID from the system.*
- **`edit / update`**: Manages customer profile updates, health scores, and lifecycle stages.
- **`addNote`**: Allows users to attach pinned or regular notes to a customer profile.

## 5. Route Mapping (`server.js`)

| Method | Route | Description |
| :--- | :--- | :--- |
| GET | `/customers` | Customer listing dashboard |
| GET | `/customers/new` | Form to add a customer |
| POST | `/customers` | Create customer action |
| GET | `/customers/:id` | Detailed customer profile & timeline |
| GET | `/customers/:id/edit` | Edit customer form |
| POST | `/customers/:id/update` | Update customer action |
| POST | `/customers/:id/delete` | Delete customer (soft delete) |
| POST | `/customers/:id/notes` | Add a customer note |

## 6. View Structure

The customer dashboard (`details.hbs`) is designed to be a unified experience:
- **Header**: Basic info and lifecycle status.
- **Timeline**: A combined view of interactions (calls, emails) and tasks.
- **Notes Section**: Pinned notes at the top, followed by chronological entries.
- **Metrics**: Health score and churn risk indicators.

## 7. Implementation Considerations

- **User Linkage**: The current backend implementation of `create_customer` requires a `user_id`. Future iterations should simplify this by allowing creation directly from email/name or providing a lookup/creation flow for users.
- **Data Conversion**: The controller handles integer conversion for IDs and health scores to match Rust backend expectations.
- **BFF Pattern**: The frontend aggregates multiple backend calls (profile + notes) into a single rendered view to reduce client-side complexity.
