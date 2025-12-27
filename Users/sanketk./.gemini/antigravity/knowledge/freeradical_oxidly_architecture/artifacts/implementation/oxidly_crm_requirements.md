# Oxidly CRM System Implementation Guide

The Customer Relationship Management (CRM) system in Oxidly serves as the central hub for managing customer interactions, data, and marketing campaigns, leveraging the FreeRadical backend's AI capabilities.

## 1. Customer Profiles
Each customer record should support at least 20 detailed fields to enable hyper-personalization.

### Core Data Fields
- **Basic Info**: Name, Email (Primary Key), Phone, Year of Birth.
- **Location**: Country, City, Address, Timezone.
- **Status**: Active, Inactive, Suspended, Banned.
- **Relational Data**:
    - **Orders**: Linked via `customer_uuid`.
    - **Products**: History of purchased products.
    - **Reviews/Ratings**: Content contributed by the customer.
    - **Interactions**: History of support tickets or campaign responses.

## 2. Analytics & Insights
- **Engagement Scoring**: Calculated based on order frequency and review activity.
- **Sentiment Analysis**: AI-powered analysis of customer reviews and support interactions.
- **Predictive Analytics**: Forecasting future purchase behavior (CLV - Customer Lifetime Value).

## 3. Campaign Management
Oxidly supports multi-channel marketing campaigns orchestrated through the CRM.

### Channel Support
- **Direct**: Email, SMS, Push Notifications.
- **In-App**: Targeted banners, personalized product carousels.
- **External**: Social media integration, Influencer tracking.

### Campaign Types
- **Affiliate/Referral**: Tracking codes and commission management.
- **Automated**: Abandoned cart recovery, welcome sequences.
- **Segment-Based**: Targeted offers for "High Value" or "At Risk" customers.

## 4. AI-Enabled CRM Features
- **AI Automated Messaging**: Natural language generation for personalized email/SMS responses.
- **AI Customer Insights**: Automated grouping of customers into personas based on behavior.
- **Semantic Search**: Searching for customers using natural language queries (e.g., "Show me customers from New York who spent over $500 last month").

## 5. UI Requirements
- **Customer List**: Searchable, filterable table with bulk actions.
- **Customer Detail View**: Comprehensive dashboard for a single user showing history and metrics.
- **Campaign Builder**: Visual interface for creating and scheduling campaigns.
