# Nexavor
Enterprise Financial Infrastructure Platform built in Rust

Nexavor is a next-generation financial infrastructure platform focused on security, auditability, deterministic execution and enterprise-grade payment services.

## Vision
Nexavor is being built as a modular financial platform capable of supporting:

- Digital Banking
- Payment Processing
- PIX
- Stablecoins
- Tokenization
- Digital Assets
- Compliance
- Enterprise APIs
- AI-assisted Financial Operations

## Architecture
- Rust (Edition 2024)
- Clean Architecture
- Domain-Driven Design (DDD)
- Service-Oriented Architecture (SOA)
- Cloud Native
- Modular Services / Cargo Workspaces
- Deterministic Core
- Security by Design
- Auditability First

## Current Status

### Sprint 0 & 1 — Foundation & Core Infrastructure
- Cargo Workspace
- Config, Shared Crates
- Custom Error Handling
- Logging & Observability (`tracing`)
- **Status:** ✅ Completed

### Sprint 2 — Authentication Service
- User Registration & Validation (Argon2id)
- JWT Access & Refresh Token Rotation / Revocation
- **APIs:** `POST /auth/register`, `POST /auth/login`, `POST /auth/refresh`
- **Status:** ✅ Completed

### Sprint 4 & 5 — Wallet & Ledger Services
- Double-Entry Accounting Ledger (Immutable audit trail)
- Wallet Service & Balances
- **Status:** ✅ Completed

### Sprint 6 — Payment Engine
- Domain entities & core business logic for Pix & Stablecoins
- Payment state management (Pending, Completed, Failed)
- Unit & integration testing flows
- **Status:** ✅ Completed

## Technologies
- Rust, Tokio, Axum, Serde, UUID, JWT, Argon2id
- SQLx, PostgreSQL, Rust Decimal
- Tracing & Observability

## Next Sprints

### Sprint 3
- Identity & Access Management (RBAC, Roles, Permissions, MFA)

### Sprint 7
- Risk Engine (AML, Fraud Detection, Compliance rules)

## Long Term Vision
Nexavor aims to become an enterprise financial operating system capable of serving Banks, Fintechs, Payment Institutions, and Enterprise Platforms.

## Repository
Private Repository

Copyright © Nexavor  
Under active development.
