# FreeRadical Admin Dashboard

Modern React admin interface for FreeRadical CMS.

## Quick Start

```bash
npm install
npm run dev
```

Opens on `http://localhost:3000`

## Features

- ✅ React 18 + TypeScript + Vite
- ✅ Tailwind CSS for styling  
- ✅ React Router for navigation
- ✅ React Query for data fetching
- ✅ API client library
- ✅ UI Components (Button, Input, Card)
- ✅ PageList with CRUD operations

## Components

### UI Components
- **Button**: Multiple variants (default, destructive, outline, ghost)
- **Input**: Form input with label and error support
- **Card**: Container component with optional title

### Pages
- **Login**: Authentication page
- **Dashboard**: Main overview
- **PageList**: Page management with table view

## Project Structure

```
src/
├── components/
│   └── ui/
│       ├── Button.tsx
│       ├── Input.tsx
│       └── Card.tsx
├── pages/
│   ├── Login.tsx
│   ├── Dashboard.tsx
│   └── PageList.tsx
├── lib/
│   └── api.ts        # API client
├── App.tsx           # Router setup
└── main.tsx          # Entry point
```

## API Integration

The admin uses an API client (`src/lib/api.ts`) that proxies requests to the backend:

```typescript
import api from './lib/api'

// Pages
api.pages.list()
api.pages.get(uuid)
api.pages.create(data)
api.pages.update(uuid, data)
api.pages.delete(uuid)

// Media
api.media.list()
api.media.upload(formData)

// Auth
api.auth.login(email, password)
```

## Development

```bash
# Install dependencies
npm install

# Start dev server (with backend proxy)
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

## Backend Connection

Vite dev server proxies `/api` requests to `http://localhost:8000`.

To change the backend URL, edit `vite.config.ts`:

```typescript
server: {
  proxy: {
    '/api': {
      target: 'http://your-backend:8000',
      changeOrigin: true
    }
  }
}
```

## Next Steps

- [ ] Install dependencies: `npm install`
- [ ] Implement authentication with JWT
- [ ] Add TipTap WYSIWYG editor
- [ ] Build page editor component
- [ ] Add media browser
- [ ] Implement dark mode

## License

MIT
