# Implementation Plan - Campaign Management

## Goal Description
Implement Campaign Management interface to allow users to create and manage marketing campaigns (Email, SMS, etc.) and the Customer Segments they target.

## User Review Required
> [!NOTE]
> I will add "Campaigns" and "Segments" as new menu items or consolidate them. For now, I will add "Campaigns" to the sidebar for direct access.

## Proposed Changes

### Oxidly Frontend

#### [MODIFY] [api.js](file:///Users/sanketk./freeradical/oxidly/services/api.js)
- Append methods to `crm` object:
    - `segments.list()`
    - `segments.create(data)`
    - `campaigns.list()`
    - `campaigns.create(data)`

#### [NEW] [controllers/campaign_controller.js](file:///Users/sanketk./freeradical/oxidly/controllers/campaign_controller.js)
- `index`: List campaigns.
- `new`: Form for new campaign (fetches segments for dropdown).
- `create`: Handle creation.
- `segments`: List segments (simple view).
- `createSegment`: Handle segment creation (simple form or modal).

#### [NEW] [views/campaigns/index.hbs](file:///Users/sanketk./freeradical/oxidly/views/campaigns/index.hbs)
- Table of campaigns with status, type, and scheduled time.

#### [NEW] [views/campaigns/form.hbs](file:///Users/sanketk./freeradical/oxidly/views/campaigns/form.hbs)
- Form to create a campaign.
- Fields: Name, Type (Email/SMS), Segment (Dropdown), Subject, Content, Scheduled At.

#### [NEW] [views/campaigns/segments.hbs](file:///Users/sanketk./freeradical/oxidly/views/campaigns/segments.hbs)
- Simple list and create form for Customer Segments.

#### [MODIFY] [server.js](file:///Users/sanketk./freeradical/oxidly/server.js)
- Register `/campaigns` and `/segments` routes.

#### [MODIFY] [views/partials/sidebar.hbs](file:///Users/sanketk./freeradical/oxidly/views/partials/sidebar.hbs)
- Add "Campaigns" link.

## Verification Plan

### Manual Verification
- Go to `/campaigns/segments` -> Create a segment (e.g., "High Value Customers").
- Go to `/campaigns/new` -> See the segment in dropdown.
- Create a campaign.
- Verify redirection to `/campaigns` and see the new campaign listed.
