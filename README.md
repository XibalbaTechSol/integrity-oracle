# 🏛️ Xibalba Integrity Sovereign Protocol (Unified Monorepo)

The comprehensive, institutional-grade infrastructure for AI Agent Trust, Reputation, and Verification.

---

## 📦 Project Structure

This monorepo consolidates the entire Integrity ecosystem:

### ⚙️ Core Engines
- **[/backend](./backend)**: The Protocol Orchestrator. Handles AIS scoring, Identity (DID/XNS), and protocol logic.
- **[/oracle-core](./oracle-core)**: The high-performance ZK-proof verification engine (Rust/Axum + Barretenberg FFI).
- **[/passport-verifier](./passport-verifier)**: Specialized verification service for agent "passports".

### ⛓️ Blockchain & Proofs
- **[/contracts](./contracts)**: Solidity smart contracts for Base L2 (IntegrityToken, StateAnchor, ReputationRegistry).
- **[/circuits](./circuits)**: Aztec Noir ZK circuits for telemetry and reputation proofs.

### 🌐 Infrastructure & Documentation
- **[/gateway](./gateway)**: Nginx mTLS gateway for zero-trust API access.
- **[/docs](./docs)**: System-wide architecture, whitepapers, and roadmap.
- **[WIKI.md](./WIKI.md)**: Deep technical reference.

---

## ⚡ Quickstart

1. **Start the Oracle Core**:
   ```bash
   cd oracle-core && cargo run
   ```

2. **Start the Protocol Backend**:
   ```bash
   cd backend && cargo run
   ```

3. **Deploy Contracts (Local)**:
   ```bash
   cd contracts && npm install && npx hardhat node
   ```

---

## 📜 License
MIT License. Built for institutional stability. Engineering the future of AI trust.
