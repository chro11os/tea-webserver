# 🫖 TEA — Telemetric Event API

**TEA** is a high-performance, event-driven, and secure real-time broadcasting API built entirely in [Rust](https://www.rust-lang.org/). Designed for low-latency systems and distributed applications, TEA acts as a smart gateway between microservices, client frontends, and event consumers—integrating seamlessly with [Laravel Echo](https://laravel.com/docs/echo), [Pusher](https://pusher.com/), and custom WebSocket infrastructures.

---

## ✨ Features

- ⚡ Ultra-fast event dispatching with `axum` (tokio-based async HTTP framework)
- 🔒 Secure secret and environment management via `.env` and `dotenv`
- 📦 JSON parsing with zero-copy serialization using `serde` & `serde_json`
- 🌐 Integrates with **Pusher** for Laravel Echo broadcasting
- 🧩 Modular architecture with separation of concerns (`routes`, `handlers`, `models`, `utils`)
- 📈 Extensible for telemetry, monitoring, and distributed tracing
- 🧪 Fully testable and built for scale (horizontal & vertical)
- 🛡️ Input sanitization and simple validation by default
- 🧱 Built with a microservice-first mindset

---

## 🔧 Built With

- [Rust](https://www.rust-lang.org/)
- [axum](https://crates.io/crates/axum) — async web framework
- [tokio](https://tokio.rs/) — async runtime
- [serde](https://serde.rs/) — JSON (de)serialization
- [reqwest](https://crates.io/crates/reqwest) — HTTP client for external API calls
- [dotenv](https://crates.io/crates/dotenv) — Environment management
- [tracing](https://crates.io/crates/tracing) — Logging and instrumentation

---

## 🚀 Quick Start

1. **Clone the repo**

```bash
git clone https://github.com/your-username/tea
cd tea

    Setup environment

cp .env.example .env
# Fill in your PUSHER credentials

    Run the server

cargo run

The server will be running on http://localhost:3000.
🛰️ Example API Request

curl -X POST http://localhost:3000/broadcast \
-H "Content-Type: application/json" \
-d '{
  "channel": "chat",
  "event": "message",
  "data": {
    "user": "neil",
    "message": "Hello from Rust!"
  }
}'

🧠 Philosophy

    TEA isn’t just a broadcasting API—it's a real-time synchronization node for your architecture. Its core values are performance, modularity, and composability. TEA can act as an autonomous microservice or be embedded within your orchestrated deployment (Docker, k8s, etc.).

🛠️ TODO

Add JWT-based auth middleware

Add optional WebSocket support via tokio-tungstenite

Healthcheck endpoints

Dockerize for production

    Prometheus / Grafana monitoring hooks

👨‍💻 Author

Built by Neil using Rust, caffeine, and a bit of chaos.
🧪 License

MIT

    “Like tea, it's powerful, refreshing, and better when shared.” ☕
