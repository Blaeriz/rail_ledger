# Rail Ledger - Frontend

A comprehensive railway management system built with SvelteKit, featuring role-based access control for Admin, Inspector, and Viewer roles.

## 🏗️ Project Structure

```
src/
├── lib/
│   ├── components/
│   │   ├── ui/                    # Reusable UI components
│   │   │   ├── Button.svelte
│   │   │   ├── Input.svelte
│   │   │   └── Card.svelte
│   │   ├── Layout.svelte          # Main layout wrapper
│   │   ├── Navbar.svelte          # Top navigation
│   │   └── Sidebar.svelte         # Side navigation
│   ├── data/
│   │   └── mockData.js            # Mock data for development
│   ├── utils/
│   │   ├── auth.js                # Authentication utilities
│   │   └── constants.js           # Application constants
│   └── server/
│       └── db/                    # Database schema and connection
├── routes/
│   ├── +page.svelte               # Root redirect page
│   ├── home/+page.svelte          # Landing page
│   ├── login/+page.svelte         # Authentication
│   ├── admin/                     # Admin role pages
│   │   ├── dashboard/+page.svelte
│   │   ├── vendors/+page.svelte
│   │   ├── batches/+page.svelte
│   │   └── users/+page.svelte
│   ├── inspector/                 # Inspector role pages
│   │   ├── dashboard/+page.svelte
│   │   ├── scan/+page.svelte
│   │   ├── inspections/+page.svelte
│   │   └── fitments/+page.svelte
│   ├── viewer/                    # Viewer role pages
│   │   ├── dashboard/+page.svelte
│   │   └── analytics/+page.svelte
│   ├── reports/+page.svelte       # Common reports page
│   └── profile/+page.svelte       # User profile
└── app.html                       # HTML template
```

## 🚀 Getting Started

1. **Install dependencies:**
   ```bash
   npm install
   ```

2. **Start development server:**
   ```bash
   npm run dev
   ```

3. **Open in browser:**
   ```
   http://localhost:5173
   ```

## 🔐 Authentication

### Demo Credentials
- **Admin**: `admin` / `admin123`
- **Inspector**: `inspector` / `insp123`
- **Viewer**: `viewer` / `view123`

### Role-Based Access
- **Admin**: Full access to all features including user management
- **Inspector**: Access to QR scanning, inspections, and fitments
- **Viewer**: Read-only access to dashboards and analytics

## 🎨 Features

### Admin Features
- **Dashboard**: Summary cards, AI alerts, quick actions
- **Vendor Management**: Add, edit, delete vendors with GST/PAN tracking
- **Batch Management**: Track batches with QR code generation
- **User Management**: Manage user roles and permissions

### Inspector Features
- **Dashboard**: Assigned inspections and alerts
- **QR Scanner**: Scan QR codes or manual batch entry
- **Inspections**: Record inspection results and status
- **Fitments**: Track fitment locations with photo uploads

### Viewer Features
- **Dashboard**: Read-only summary with system alerts
- **Analytics**: Performance charts and compliance metrics

## 🛠️ Technical Stack

- **Framework**: SvelteKit
- **Styling**: CSS with custom design system
- **State Management**: Svelte stores and localStorage
- **Authentication**: Role-based with localStorage persistence
- **Data**: Mock data for development

## 📁 Key Files

### Utilities
- `lib/utils/auth.js` - Authentication helper functions
- `lib/utils/constants.js` - Application constants and enums
- `lib/data/mockData.js` - Mock data for development

### Components
- `lib/components/Layout.svelte` - Main layout wrapper
- `lib/components/ui/` - Reusable UI components

### Routes
- `routes/+page.svelte` - Root redirect based on auth status
- `routes/login/+page.svelte` - Authentication page
- `routes/home/+page.svelte` - Public landing page

## 🔧 Development

### Adding New Pages
1. Create a new folder in `routes/`
2. Add `+page.svelte` file
3. Import and use `Layout` component for authenticated pages
4. Add route to sidebar navigation if needed

### Adding New Components
1. Create component in `lib/components/`
2. Use TypeScript for better type safety
3. Follow existing naming conventions
4. Export from `lib/index.ts` if needed

### Styling Guidelines
- Use CSS custom properties for consistent theming
- Follow mobile-first responsive design
- Use semantic HTML elements
- Maintain accessibility standards

## 🚀 Deployment

1. **Build for production:**
   ```bash
   npm run build
   ```

2. **Preview production build:**
   ```bash
   npm run preview
   ```

## 📝 Notes

- The `pages/` folder was removed as SvelteKit only uses `routes/`
- All authentication logic is centralized in `lib/utils/auth.js`
- Mock data is used for development - replace with API calls in production
- The application uses localStorage for session management