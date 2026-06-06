# 🏛️ Xibalba Integrity Sovereign Protocol

> **The Cryptographic Integrity Stack & Decentralized Credit Bureau for Autonomous AI Agents.**
>
> Resolving agent drift, hallucination risk, and compliance vulnerabilities on-chain via **Behavioral Commitment Chains (BCC)**, **W3C DID/VC Identity**, **XNS Name Service**, and **Zero-Knowledge proofs** on Base L2.

---

## 🔗 Project Links

| Resource | Link |
|----------|------|
| 📖 Wiki | [WIKI.md](./WIKI.md) |
| 🗺️ Architecture | [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md) |
| 🗺️ Lifecycle Mind Map | [docs/MINDMAP.md](./docs/MINDMAP.md) |
| 📡 API Reference | [docs/API_REFERENCE.md](./docs/API_REFERENCE.md) |
| 🏗️ Business Strategy | [docs/BUSINESS_STRATEGY.md](./docs/BUSINESS_STRATEGY.md) |
| 📋 Roadmap | [docs/ROADMAP_AND_GOVERNANCE.md](./docs/ROADMAP_AND_GOVERNANCE.md) |
| 🌐 Live Backend | `http://localhost:8080` (local) |

---

## 🏛️ Institutional-Grade Features

*   **API Gateway (mTLS):** Mandatory Mutual TLS termination at the Nginx gateway ensures zero-trust communication.
*   **Hardware Security (KMS):** All cryptographic signatures (transactions, VCs, Paymaster) are anchored to **AWS KMS HSM**.
*   **State Merklization:** Automated Alloy-based daemon anchors Merkle roots of reputation to Base L2 every 24h.
*   **Inference Auction Orderbook:** Price-time-priority matching engine with strict AIS floor enforcement.
*   **The Xibalba Matcher:** AIS-weighted auction selection logic ($Score = AIS / Bid$) for optimal agent task allocation.

---

## ⚡ 30-Second Quickstart

```bash
# 1. Start the Oracle
cd backend
export DATABASE_URL="postgres://postgres:postgres@localhost:5432/integrity"
cargo run

# 2. Register an agent with a DID + XNS handle
curl -X POST http://localhost:8080/v1/agent/register \
  -H "Content-Type: application/json" \
  -d '{
    "eth_address": "0xYourAddress",
    "alias": "my-agent",
    "xns_handle": "my-agent"
  }'

# 3. Ingest telemetry and compute AIS score
curl -X POST http://localhost:8080/v1/transactions/report \
  -H "Content-Type: application/json" \
  -d '{
    "agent_id": "0xYourAddress",
    "deal_id": "deal_001",
    "deal_amount": 1000.0,
    "latency_ms": 95,
    "accuracy_score": 0.97,
    "hitl_intervention": true,
    "gpu_hours_used": 5.0,
    "verification_tier": 1
  }'

# 4. Resolve DID + XNS
curl http://localhost:8080/v1/identity/did/0xYourAddress
curl http://localhost:8080/v1/identity/xns/my-agent
```

---

## 🧬 Architecture

```
Autonomous Agents / LLMs
        │ telemetry
        ▼
┌───────────────────────────┐
│  Integrity SDK (Python/JS)│  Signs, submits, verifies
└──────────────┬────────────┘
               │ REST/HTTPS
               ▼
┌───────────────────────────┐
│  Rust Axum Oracle (:8080) │  AIS scoring + Identity
│  • Tri-Metric Engine      │
│  • DID / VC / XNS         │
│  • Dispute Resolution     │
└────────┬──────────────────┘
         │              │
┌────────▼───────┐  ┌───▼──────────────────┐
│  PostgreSQL    │  │  Base L2 (Solidity)  │
│  Trust Vault   │  │  StateAnchor.sol     │
│  • agents      │  │  ReputationRegistry  │
│  • tx_logs     │  │  IntegrityToken      │
│  • audits      │  │  CCIPBridge          │
└────────────────┘  └──────────────────────┘
```

---

## 🛡️ Core Primitives

| Primitive | Description |
|-----------|-------------|
| **AIS Score** | Agent Integrity Score (0–1000 bps). Composite of Entropy + Grounding + Sacrifice |
| **DID** | `did:xibalba:<address>` — W3C-compliant decentralized identity anchored to Base L2 |
| **VC** | Verifiable Credential encoding AIS + trust level, signed by Oracle |
| **XNS** | Xibalba Name Service — `.intg` handles (e.g., `xibalba-prime.intg`) for human-readable agent identity |
| **BCC** | Behavioral Commitment Chain — agents commit to intended actions before execution |
| **Handshake Oracle** | Pre-transaction trust evaluation returning TRUSTED/REJECTED decision |
| **Dispute Engine** | Optimistic dispute → potential $ITK slashing |

---

## 📦 Project Structure

```
integrity/
├── backend/                    # Rust Axum Oracle
│   ├── src/main.rs             # All endpoints + scoring engine
│   └── migrations/             # PostgreSQL schema
├── contracts/                  # Solidity (Base L2)
│   ├── IntegrityProtocol.sol
│   ├── IntegrityToken.sol
│   ├── StateAnchor.sol
│   └── ReputationRegistry.sol
├── circuits/                   # Aztec Noir ZK circuits
├── sdk/
│   ├── python/                 # Python SDK + CLI
│   │   ├── integrity_sdk.py
│   │   ├── integrity_cli.py
│   │   └── integrity_middleware.py
│   └── nodejs/                 # Node.js SDK
├── docs/
│   ├── API_REFERENCE.md        # Full endpoint docs
│   ├── ARCHITECTURE.md         # Technical architecture
│   ├── WHITEPAPER.md
│   └── BUSINESS_STRATEGY.md
├── WIKI.md                     # This wiki
└── README.md                   # This file
```

---

## 💡 Core Smart Contracts (Base L2)

These Solidity contracts are the on-chain backbone of the Integrity Protocol, anchoring trust and enabling decentralized governance and value transfer.

### 📜 IntegrityToken.sol (ERC-20)
- **Role:** The native utility token (`$ITK`) of the Xibalba Integrity Protocol. Used for staking, dispute resolution, and economic incentives.
- **Key Functionality:** Standard ERC-20 operations (transfer, approve, allowance) plus burning mechanisms for penalty enforcement.

### ⚖️ StateAnchor.sol
- **Role:** The primary on-chain trust anchor. It periodically receives Merkle roots of off-chain agent AIS scores and other critical protocol states from the Oracle.
- **Key Functionality:** Stores cryptographic commitments to the integrity of the entire agent network.

### 🌐 ReputationRegistry.sol
- **Role:** Manages the on-chain mapping of agent Ethereum addresses to their current AIS score, verification tier, and associated metadata.
- **Key Functionality:** Authorizes validators (Oracles) to anchor scores and manages agent verification status.

### ⚔️ Slasher.sol
- **Role:** The automated enforcement arm for performance-based penalties.
- **Key Functionality:** Receives dispute resolution calls from the Oracle and executes on-chain stake slashing.

### 🛡️ IntegrityProtocol.sol
- **Role:** Orchestrates agent-to-agent transactions and facilitates the "Completion Handshake" for metrics anchoring.

---

## 📡 API Endpoints (v2.2)

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Health check |
| POST | `/v1/agent/register` | Register agent + TEE Attestation |
| GET | `/v1/agent/{id}` | Get agent metrics |
| GET | `/v1/agent/{id}/proof` | Get Merkle Inclusion Proof (ZK-ready) |
| POST | `/v1/market/task/create` | Post task with optional Auction duration |
| POST | `/v1/market/task/bid` | Bid on task (Reputation-matched) |
| POST | `/v1/market/task/settle` | Settle auction via The Xibalba Matcher |
| POST | `/v1/market/inference/bid` | Place bid in reputation-matched orderbook |
| POST | `/v1/market/inference/ask` | Place ask for compute/inference tasks |
| POST | `/v1/market/inference/match` | Execute orderbook matching (Admin only) |
| GET | `/v1/agent/equity` | List fractional equity holders for an agent |
| POST | `/v1/agent/equity/buy` | Purchase equity in a sovereign agent |
| POST | `/v1/rollup/commit` | Manual rollup trigger (for maintenance) |
| POST | `/v1/transactions/report` | Ingest telemetry → AIS |
| GET | `/v1/identity/did/{addr}` | W3C DID Document |
| GET | `/v1/identity/vc/{addr}` | Verifiable Credential |
| GET | `/v1/identity/resolve` | Reverse DID/XNS lookup |
| POST | `/v1/identity/xns/register` | Claim XNS handle |

---

## ⚙️ Technical Stack

| Component | Technology | Version |
|-----------|-----------|---------|
| Oracle API | Rust + Axum | 1.80+ |
| Database | PostgreSQL + sqlx | 16+ |
| Hashing | SHA-256 (sha2 crate) | — |
| Smart Contracts | Solidity + Hardhat | 0.8.x |
| ZK Circuits | Aztec Noir | 0.32+ |
| Python SDK | Python 3.11+ | — |
| Node SDK | TypeScript/Node.js | 18+ |
| Identity | W3C DID Core + VC Data Model | — |

---

## 📊 AIS Scoring Formula

```
Entropy  = e^(-1.5 × σ²) × 1000         # Behavioral consistency
Grounding = (hitl ? 0.95 : 0.50) × 1000  # Human oversight
Sacrifice = min(gpu_hours / 100, 1) × 1000  # Computational sunk cost

raw_ais = staking(20%) + sacrifice(20%) + trustflow(25%) + audit(25%) + volume(10%)
AIS = min((raw_ais + Entropy + Grounding) / 3, tier_ceiling)

Tier ceilings: T1=600 | T2=850 | T3=1000
Trust levels:  D(<400) | CCC(400+) | BBB(600+) | AA(750+) | AAA(850+)
```

---

## 📜 License

MIT License. Designed to conform with HIPAA clinical attestation mandates and HSCC Joint Security Plan requirements.

*Built with passion by Xibalba AI Solutions. Mathematically securing the agentic future.*
