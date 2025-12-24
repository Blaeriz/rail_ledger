# 🚂 Rail Ledger

> **A Smart Railway Asset Management & Quality Control Platform**

Rail Ledger is a comprehensive railway component lifecycle management system built for **SIH 2025**. It provides end-to-end traceability from production to deployment, featuring AI-powered analytics, role-based dashboards, and QR-based asset tracking.

---

## 📋 Table of Contents

- [Features](#-features)
- [Tech Stack](#-tech-stack)
- [Architecture](#-architecture)
- [Getting Started](#-getting-started)
- [Environment Variables](#-environment-variables)
- [Database Setup](#-database-setup)
- [Available Scripts](#-available-scripts)
- [Project Structure](#-project-structure)
- [Role-Based Dashboards](#-role-based-dashboards)
- [API Endpoints](#-api-endpoints)
- [TUI (Terminal Interface)](#-tui-terminal-interface)
- [AI Reports](#-ai-reports)
- [Contributing](#-contributing)

---

## ✨ Features

### Core Functionality
- **🔐 QR Code Generation & Tracking** — Store production facility data, quality check pipeline, batch info, production dates, and fitment details
- **📦 Batch Management** — Complete lifecycle tracking from production to fitment
- **🏭 Vendor Management** — Track vendor performance, audit schedules, and batch history
- **📊 Inspection Reports** — Digital inspection workflows with pass/fail status tracking
- **🎫 Ticketing System** — Issue tracking with priority levels and resolution workflows

### Advanced Features
- **🤖 AI-Powered Reports** — Intelligent insights using Google Gemini API for:
  - Quality Analysis Reports
  - Predictive Maintenance Analysis
  - Vendor Performance Assessment
  - Safety Risk Assessment
- **🗺️ Geographic Heatmaps** — Visualize asset distribution and issues
- **📈 Analytics Dashboard** — Real-time metrics with Chart.js visualizations
- **🔔 Event Logging & Notifications** — System-wide activity tracking

### Unique Differentiators
- Complete QR → Laser Engraving pipeline integration
- Role-based access control (Admin, Inspector, Viewer)
- Rust-based Terminal UI for system administrators
- Real-time data synchronization

---

## 🛠️ Tech Stack

| Category | Technology |
|----------|------------|
| **Framework** | SvelteKit 2.x with Svelte 5 |
| **Language** | TypeScript |
| **Styling** | TailwindCSS 4.x |
| **Database** | PostgreSQL (Neon / Docker) |
| **ORM** | Drizzle ORM |
| **AI Integration** | Google Gemini API |
| **Visualizations** | Chart.js, D3.js, TopoJSON |
| **TUI** | Rust with Ratatui |
| **ML** | Xenova Transformers (ONNX) |

---

## 🏗️ Architecture

```
rail_ledger/
├── src/                    # SvelteKit Application
│   ├── routes/             # Page routes & API endpoints
│   │   ├── admin-dashboard/    # Admin features
│   │   ├── inspector-dashboard/ # Inspector features
│   │   ├── viewer-dashboard/   # Read-only views
│   │   ├── api/               # RESTful API routes
│   │   └── login/             # Authentication
│   └── lib/                # Shared utilities
│       ├── components/     # Reusable Svelte components
│       ├── services/       # Business logic
│       └── types.ts        # TypeScript definitions
├── tui/                    # Rust Terminal Interface
│   └── src/               # TUI source code
├── app/                   # Additional app modules
├── db/                    # Database design docs
└── static/                # Static assets
```

---

## 🚀 Getting Started

### Prerequisites

- **Node.js** ≥ 18.x
- **npm** or **pnpm**
- **Docker** (for local PostgreSQL)
- **Rust** (optional, for TUI)

### Installation

```bash
# Clone the repository
git clone https://github.com/your-username/rail_ledger.git
cd rail_ledger

# Install dependencies
npm install

# Start the database
npm run db:start

# Push schema to database
npm run db:push

# (Optional) Seed with sample data
psql -h localhost -U root -d local -f seed-data.sql

# Start development server
npm run dev
```

Visit **http://localhost:5173** to access the application.

---

## 🔐 Environment Variables

Create a `.env.local` file in the root directory:

```env
# Database (using Neon or local PostgreSQL)
DATABASE_URL=postgresql://root:mysecretpassword@localhost:5432/local

# AI Reports (Google Gemini)
GEMINI_API_KEY=your_gemini_api_key_here
```

### Optional Variables

```env
# TUI Configuration
RAIL_LEDGER_API_BASE=http://localhost:5173
REFRESH_INTERVAL_MS=30000
```

---

## 🗄️ Database Setup

### Using Docker (Recommended for Development)

```bash
# Start PostgreSQL container
npm run db:start

# Apply schema migrations
npm run db:push

# View database in browser
npm run db:studio
```

### Database Schema

| Table | Description |
|-------|-------------|
| `user_info` | User accounts with roles (admin/inspector/viewer) |
| `vendor_info` | Vendor details, GST, PAN, audit history |
| `batch_info` | Production batches with QC status and fitment data |
| `inspection_reports` | Inspection records linked to batches |

---

## 📜 Available Scripts

| Command | Description |
|---------|-------------|
| `npm run dev` | Start development server |
| `npm run build` | Build for production |
| `npm run preview` | Preview production build |
| `npm run check` | TypeScript type checking |
| `npm run lint` | Run ESLint + Prettier checks |
| `npm run format` | Auto-format code |
| `npm run db:start` | Start PostgreSQL Docker container |
| `npm run db:push` | Push schema to database |
| `npm run db:generate` | Generate migrations |
| `npm run db:migrate` | Run pending migrations |
| `npm run db:studio` | Open Drizzle Studio GUI |

---

## 🖥️ Role-Based Dashboards

### 👑 Admin Dashboard (`/admin-dashboard`)
Full system access including:
- **Overview** — System-wide statistics
- **Batches** — CRUD operations on production batches
- **Vendors** — Vendor management and audit scheduling
- **Users** — User account management
- **Reports** — All inspection reports
- **Tickets** — Issue tracking and resolution
- **Analytics** — Advanced data visualizations
- **AI Reports** — AI-generated insights
- **Heatmap** — Geographic asset distribution
- **Settings** — System configuration

### 🔍 Inspector Dashboard (`/inspector-dashboard`)
Inspection-focused features:
- Batch inspection workflows
- Report creation and editing
- Quality status updates
- AI-assisted analysis

### 👁️ Viewer Dashboard (`/viewer-dashboard`)
Read-only access for:
- Viewing batch status
- Reading reports
- Monitoring system metrics
- Ticket visibility

---

## 🔌 API Endpoints

### Users
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/users` | List all users |
| GET | `/api/users/[id]` | Get user by ID |

### Vendors
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/vendors` | List all vendors |
| GET | `/api/vendors/[id]` | Get vendor by ID |

### Batches
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/batches` | List all batches |
| GET | `/api/batches/[id]` | Get batch by ID |
| GET | `/api/batches/qr_hash?qr_hash=...` | Lookup by QR hash |

### Reports
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/reports` | List inspection reports |
| GET | `/api/reports/[id]` | Get report by ID |

### AI & Analytics
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/summary` | System summary stats |
| POST | `/api/ai` | AI query endpoint |
| GET | `/api/ai-reports` | Pre-generated AI reports |
| GET | `/api/heatmap` | Geographic distribution data |
| GET | `/api/metrics` | Prometheus-format metrics |

---

## 🖥️ TUI (Terminal Interface)

A Rust-based read-only terminal interface for system administrators.

### Features
- **Tab-based navigation** — Overview, Batches, Vendors, Reports, Users, Tickets, AI Insights
- **Keyboard controls** — Full keyboard navigation
- **Live updates** — Configurable refresh intervals
- **QR Tools** — Lookup batches by QR hash

### Running the TUI

```bash
cd tui
cargo run --release
```

### Controls
| Key | Action |
|-----|--------|
| `q` | Quit |
| `Tab` | Next tab |
| `Shift+Tab` | Previous tab |

---

## 🤖 AI Reports

Powered by **Google Gemini API**, generating intelligent insights:

| Report Type | Description |
|-------------|-------------|
| **Quality Analysis** | Batch quality trends and anomaly detection |
| **Predictive Maintenance** | Proactive maintenance recommendations |
| **Vendor Performance** | Supplier rating and comparison |
| **Safety Risk Assessment** | Safety hazard identification |

Each report includes:
- ✅ AI-generated insights and recommendations
- 📊 Confidence scores
- 🎯 Priority levels
- 📈 Interactive visualizations

---

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## 📄 License

This project is developed for **Smart India Hackathon 2025**.

---

<div align="center">
  
**Built with ❤️ by the best team ever <3**

</div>
