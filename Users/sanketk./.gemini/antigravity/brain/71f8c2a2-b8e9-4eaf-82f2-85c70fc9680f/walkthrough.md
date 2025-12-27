# Oxidly Cloud Platform - Foundation Walkthrough

We have successfully initialized the **Oxidly Cloud Platform**, the web frontend for the FreeRadical CMS.

## Achievements
- [x] **Project Setup**: Initialized Node.js/Express project with Handlebars templates.
- [x] **Docker**: Created `Dockerfile` for containerization.
- [x] **Authentication UI**: Implemented Login, Signup, and Email Verification pages.
- [x] **Dashboard Shell**: Created a responsive dashboard layout with a sidebar and header.
- [x] **API Client**: Configured `services/api.js` to connect with the FreeRadical v1.7.0 backend.

## Screenshots

### Home Page
![Home Page](/Users/sanketk./freeradical/oxidly/views/home.hbs)
*(Template source)*

### Dashboard
![Dashboard](/Users/sanketk./freeradical/oxidly/views/dashboard/index.hbs)
*(Template source)*

## Verification
To run the server locally:
```bash
cd oxidly
npm install
npm run dev
```
Visit `http://localhost:3000` to see the application.

## Next Steps
We are now ready to move to **Phase 2: Site & Content Management**.
