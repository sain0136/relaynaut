# 🛰️ Relaynaut

**Relaynaut** is a Rust-first, low-latency overlay networking and data fan-out system inspired by Tailscale and modern high-frequency trading infrastructure.

It provides:

- Secure, peer-to-peer connectivity with automatic relay fallback (like DERP)
- Real-time latency metrics (p50/p95/p99, jitter, loss)
- Tunable socket, MTU, and CPU parameters for tail-latency optimization
- A web dashboard (Vue 3 + Vite) for visualization and PDF reports
- An experimental **HFT Pack** for synthetic tick streams and deterministic replays

Relaynaut is designed to demonstrate:

- Expertise in overlay networking and NAT traversal
- Hands-on mastery of p99 performance engineering
- A realistic path toward commercial low-latency data infrastructure

---

## 🧭 Current Status

🚧 _Phase 1 – Foundation setup (workspace, CI, dashboard skeleton)_  
Follow the [Project Roadmap](docs/roadmap.md) for progress tracking.

---

## ⚙️ Tech Stack

**Backend:** Rust (Tokio, Axum, metrics, Prometheus exporter)  
**Frontend:** Vue 3 + Vite + Pinia + Chart.js  
**Observability:** Prometheus + Grafana  
**Storage:** SQLite, MinIO (for reports)

---

## 📊 Early Goals

1. Measure p99 latency between peers across NATs
2. Visualize direct vs relay paths
3. Deliver first HFT replay demo by Week 6

---

## 💡 Why "Relaynaut"?

Because every peer is an astronaut exploring the mesh — relaying data through space, chasing the lowest p99 in the galaxy. 🪐

---

## 🪪 License

Source available under the **Business Source License 1.1 (BUSL-1.1)**.  
Commercial use prohibited without a separate license agreement.

© 2025 IMU Technology Solutions. All rights reserved.
