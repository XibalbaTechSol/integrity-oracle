# Integrity Oracle: The Foundational Trust Engine

The Integrity Oracle is the high-performance, domain-agnostic Layer 0 infrastructure for AI agent trust, reputation, and cryptographic verification within the Integrity Protocol ecosystem.

## Overview
As the core truth engine, the Oracle is responsible for:
- **ZK-Proof Verification**: Authenticating telemetry using Aztec Noir circuits and the Barretenberg backend.
- **AIS Scoring**: Computing and anchoring Agent Integrity Scores (AIS).
- **L2 Anchoring**: Periodically rolling up state to Base L2 for immutable auditing.

---

## 🏗️ Architecture

The Oracle is organized into specialized, high-performance components:

### Core Components
- **`oracle-core/`**: High-performance Rust verification engine utilizing FFI for native ZK-proof verification.
- **`backend/`**: Protocol orchestrator managing telemetry ingestion, AIS scoring, and state anchoring.
- **`circuits/`**: Aztec Noir ZK circuits for telemetry integrity and reputation scoring.
- **`passport-verifier/`**: Service for verifying cryptographic agent "passports".

---

## 🚀 Getting Started

### Prerequisites
- [Rust](https://rustup.rs/)
- [Foundry](https://book.getfoundry.sh/) (for contract interactions)

### Running Core Services
1. **Oracle Engine:**
   ```bash
   cd oracle-core
   cargo run
   ```
2. **Protocol Backend:**
   ```bash
   cd backend
   cargo run
   ```

---

## 🛠️ Infrastructure Requirements
- **Database**: PostgreSQL 15+ required for backend telemetry storage.
- **FFI**: Native Barretenberg binaries must be accessible for the `oracle-core` FFI verification to function.

## 🛠️ Troubleshooting
- **Build Errors**: Ensure `sqlx-cli` is installed to manage database migrations.
- **FFI Errors**: Verify that the Barretenberg static library (`libbb_rs.a`) is correctly linked in your system path.

---

## 🤝 Contribution Guidelines
Contributions follow the standard Rust community standards. Please run `cargo fmt` and `cargo test` before submitting PRs.

---

## 📜 License
This project is licensed under the **Apache License 2.0**.
