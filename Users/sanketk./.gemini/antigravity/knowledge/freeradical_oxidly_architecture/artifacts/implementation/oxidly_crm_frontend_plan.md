# Implementation Plan - Customer CRM Interface

## Goal Description
Implement the Customer Relationship Management (CRM) interface in the Oxidly frontend. This will allow users to manage customers, view their profiles, interactions, and tasks, leveraging the existing backend CRM endpoints.

## User Review Required
> [!NOTE]
> No critical user review is required as this follows the standard pattern of the application.

## Proposed Changes

### Oxidly Frontend

#### [MODIFY] api.js
- Add `crm` object with methods:
    - `customers.list(filters)`
    - `customers.get(id)`
    - `customers.create(data)`
    - `customers.update(id, data)`
    - `customers.delete(id)`
    - `customers.getTimeline(id)`
    - `customers.addNote(id, data)`
    - `customers.getNotes(id)`

#### [NEW] crm_controller.js
- Implement `index` to list customers.
- Implement `show` to view customer details (combines profile, activity timeline, notes).
- Implement `new` and `create` for adding customers.
- Implement `edit` and `update` for modifying customers.

#### [NEW] views/crm/index.hbs
- List view with table of customers.
- Filters for lifecycle stage, churn risk.

#### [NEW] views/crm/details.hbs
- comprehensive view of a single customer.
- **Combine** profile info, interaction timeline, and notes into one dashboard-like view.

#### [NEW] views/crm/form.hbs
- Form for creating/editing customers.

#### [MODIFY] server.js
- Register `/customers` routes pointing to `crmController`.

#### [MODIFY] sidebar.hbs
- Update "Customers" link to `/customers`.
- Add active state logic.

## Verification Plan

### Manual Verification
- Start the server (`npm start` or via docker if running).
- Navigate to `/customers`.
- Verify the list loads (empty initially or with seed data).
- Click "New Customer".
- specific form fields validation.
- Create a customer.
- Verify redirection to list or details.
- detailed view shows the created customer.
- Add a note to the customer.
- Verify the note appears in the timeline/notes section.
