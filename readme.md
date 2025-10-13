# Vigil

> Vigil — an open, extensible observability backend, implemented in Rust.

Vigil collects, indexes, analyzes, and acts on logs, metrics, and events. The project focuses on modularity, workflow-driven alerting/escalation policies, and AI-assisted analysis for faster incident detection and root-cause hints. In the future, Vigil will support decentralized storage backends and peer-based replication rather than a centralized database.

## Vision and goals

- Provide a lightweight, production-ready observability backend in Rust with secure defaults.
- Full feature parity (configurable) with BetterStack-style alerting and dashboards, while keeping the codebase open and extensible.
- Native support for escalation policies and workflows (on-call rotation, notifications, auto-escalation).
- Integrate AI for log analysis, anomaly detection, and automated pain-point summarization.
- Support multiple storage backends; long-term roadmap includes decentralized DB options.

## Key features

- Ingest: structured logs, plain text logs, metrics (Prometheus-style), and traces (optional).
- Storage: pluggable backends (Postgres via Diesel now; future: decentralized stores).
- Querying: fast, typed query surface for logs and metrics with pagination and aggregation.
- Alerting & workflows: flexible rules, threshold and anomaly detection, escalation policies, and webhooks.
- AI-assisted analysis: contextual log summarization, anomaly explanation, and suggested remediation steps.
- Security & observability: RBAC-ready, audit logs, rate-limiting, and telemetry for Vigil itself.

## Project architecture

**DataBase Architecture**

![alt text](db_schema.png)

**Backend Architecture**

![alt text](be_arch.png)

## Quickstart (local development)

Prerequisites

- Rust toolchain (stable; rustup recommended).
- PostgreSQL (for local development) with a database for Vigil.
- Diesel CLI (optional) for running migrations: `cargo install diesel_cli --no-default-features --features postgres`

Environment

1. Create a local Postgres database and note connection URL (example):

	 postgres://vigil:password@localhost/vigil_dev

2. Set environment variables used by the API and store crates (example):

	 RUST_LOG=info
	 DATABASE_URL=postgres://vigil:password@localhost/vigil_dev

Run migrations

	 diesel migration run --manifest-path store/Cargo.toml

Run the API server (from workspace root)

	 cargo run -p api --bin api

You should see the server bind to the configured address (default in config or env).

## Configuration

Configuration is via environment variables and (optionally) a config file. Important variables:

- DATABASE_URL — Postgres connection string used by Diesel.
- VIGIL_ADDR — address to bind the API server (default: 127.0.0.1:8080).
- RUST_LOG — logging level for development.

Feature flags

- `store` uses Diesel with Postgres enabled by default in the current manifest. If you want SQLite or another backend, update `store/Cargo.toml` and adjust migrations accordingly.

## Environment variables

Vigil uses a single `.env` file at the workspace root for local development. A sample file is provided as `.env.example`.

Steps to use:

1. Copy `.env.example` to `.env` at the repository root:

```bash
cp .env.example .env
# edit .env and fill in real credentials
```

2. Start the API (it will load `.env` when it starts):

```bash
cargo run -p api
```

Important variables (from `.env.example`):

- `DATABASE_URL` — Postgres connection string used by Diesel.
- `JWT_SECRET` — secret key used to sign JWT tokens. Keep this secret in production.
- `JWT_EXPIRY_SECONDS` — token expiry in seconds (defaults to `3600` if not set).

For production, do not use `.env` files — set environment variables in your deployment platform or use a secrets manager.


## AI integration notes

Vigil integrates AI modules for analysis and pain-point recognition. Design notes:

- AI runs as an asynchronous service that consumes enriched log/metric events or query results, returning:
	- Summaries of incidents
	- Suggested root causes (with confidence scores)
	- Suggested relevant alerts or runbook steps
- Integration approaches:
	- Local in-process model (for privacy/offline use)
	- External model endpoint (OpenAI/Anthropic/etc.) — ensure secrets are kept out of source control
	- Hybrid plugin architecture for future model providers

Privacy & safety

- Sanitize logs before sending to external services.
- Support configurable sampling and scrubbers for PII and secrets.

## Alerting, escalation policies & workflows

Vigil models workflows as first-class objects:

- Policy: a set of conditions and priorities for firing alerts.
- Workflow: flow that runs when a policy triggers. Workflows can:
	- Notify a channel (email, Slack, PagerDuty, SMS, webhook)
	- Start an on-call rotation escalation
	- Create a ticket in tracking systems
	- Trigger automated remediation jobs
- Escalation rules:
	- Immediate notification to primary on-call
	- After X minutes, escalate to secondary
	- After Y minutes, trigger team-wide paging

Example YAML policy snippet

```yaml
policy:
	name: High error rate
	conditions:
		- type: metric_threshold
			metric: http_errors
			threshold: 100
			window: 5m
	workflow:
		- notify: slack:#ops
		- escalate_after: 10m -> notify: pagerduty:service-id
```

## Developer notes

- Use `cargo fmt` and `cargo clippy` while developing.
- Run tests with `cargo test --workspace`.
- Migrations are under `store/migrations/`. Keep them idempotent and tested.

Common commands

```bash
# Build the workspace
cargo build --workspace

# Run the API
cargo run -p api

# Run tests
cargo test --workspace

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings
```

## Roadmap (short and long term)

Short-term (0–6 months)

- Harden API authentication and RBAC
- Add rich alerting channels (Slack, PagerDuty, Email)
- Implement basic AI log summarization
- Add more migrations and production-ready SQL
- Pluggable storage adapters (S3, decentralized DB backends)
- Advanced ML anomaly detection with streaming workflows
- GUI dashboard for queries & incident management

Mid-term (6-12 months)

- Decentralized database backend (peer-to-peer replication, CRDTs)
- Community plugin ecosystem (inputs, transforms, storage)
- Hosted and self-hosted deployment guides and Helm charts

## Contributing

Contributions are welcome. Please open issues or PRs. Follow these guidelines:

- Keep changes small and focused.
- Add tests for behavior changes.
- Document configuration and environment changes.

## 🗺️ Project Roadmap

| Timeline | Milestone | Status |
|-----------|------------|--------|
| **2025-10-05** | Diesel + Postgres Integration | ✅ Completed |
| **2025-10-08** | README, Project Vision & Quickstart | ✅ Completed |
| **2025-10-xx** | Store Schema & Migrations | ☐ Planned |
| **2025-10-xx** | Basic frontend implementation (For more info check vigil frontend repository) | ☐ Planned |
| **2025-11-xx** | Testcases implementation and mandatory check | ☐ Planned |
| **2025-11-xx** | Alerting Channels (Slack / PagerDuty) | ☐ Planned |
| **2025-11-xx** | AI-Assisted Anomaly Detection | ☐ Planned |
| **2025-12-xx** | SDK Implementation and support | ☐ Planned |
| **2025-12-xx** | GUI Dashboard & Incident Timeline | ☐ Planned |
| **2025-12-xx** | On-Call Rotation & Escalation Policies | ☐ Planned |
| **2026-01-xx** | Pluggable Storage Adapters (e.g., S3) | ☐ Planned |
| **2026-02-xx** | Advanced ML-Based Anomaly Detection | ☐ Planned |
| **TBD** | Decentralized DB Backend (CRDTs) | ☐ Future |


*Note* - This readme is going to evolve with the project and an seperate documentation with sdk will be made