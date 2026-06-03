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
| 📡 API Reference | [docs/API_REFERENCE.md](./docs/API_REFERENCE.md) |
| 🏗️ Business Strategy | [docs/BUSINESS_STRATEGY.md](./docs/BUSINESS_STRATEGY.md) |
| 📋 Roadmap | [docs/ROADMAP_AND_GOVERNANCE.md](./docs/ROADMAP_AND_GOVERNANCE.md) |
| 🌐 Live Backend | `http://localhost:8080` (local) |

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
- **Key Functionality:** Stores cryptographic commitments to the integrity of the entire agent network, allowing for light-client verification without re-computing all off-chain scores.

### 🌐 ReputationRegistry.sol
- **Role:** Manages the on-chain mapping of agent Ethereum addresses to their current verification tier and associated metadata hashes.
- **Key Functionality:** Allows for transparent lookup of an agent's on-chain tier status and links to their off-chain DID documents, facilitating trust evaluation for other smart contracts and external protocols.

### 🛡️ IntegrityProtocol.sol (ERC-8004 Compliant)
- **Role:** The main protocol entry point for on-chain interactions, including agent registration, staking events, and dispute finalization. Designed to be compatible with ERC-8004 (Account Abstraction).
- **Key Functionality:** Orchestrates interactions between the other core contracts, enforcing protocol rules and updating agent states based on verified off-chain events.

---

## 📡 API Endpoints (v2.1)

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Health check |
| POST | `/v1/agent/register` | Register agent + assign DID |
| GET | `/v1/user/agents` | List all agents |
| GET | `/v1/agent/{id}` | Get agent metrics |
| POST | `/v1/agent/handshake` | Trust evaluation |
| POST | `/v1/transactions/report` | Ingest telemetry → AIS |
| POST | `/v1/disputes/raise` | Raise dispute |
| POST | `/v1/disputes/resolve` | Resolve + slash |
| GET | `/v1/identity/did/{addr}` | W3C DID Document |
| GET | `/v1/identity/vc/{addr}` | Verifiable Credential |
| GET | `/v1/identity/resolve` | Reverse DID/XNS lookup |
| GET | `/v1/identity/xns/{handle}` | XNS handle resolution |
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
