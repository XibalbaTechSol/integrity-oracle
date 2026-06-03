# 🏛️ Integrity Protocol Wiki

> **Xibalba AI Solutions — Cryptographic Trust Infrastructure for Autonomous AI Agents**
> 
> Version 2.1 | Last Updated: June 2026

---

## Table of Contents

1. [What is the Integrity Protocol?](#1-what-is-the-integrity-protocol)
2. [Quick Start](#2-quick-start)
3. [Core Concepts](#3-core-concepts)
4. [API Reference Summary](#4-api-reference-summary)
5. [Agent Registration Guide](#5-agent-registration-guide)
6. [Telemetry & Scoring Guide](#6-telemetry--scoring-guide)
7. [Identity: DID, VC & XNS](#7-identity-did-vc--xns)
8. [Dispute Resolution](#8-dispute-resolution)
9. [SDK Integration](#9-sdk-integration)
10. [Database Reference](#10-database-reference)
11. [Deployment](#11-deployment)
12. [Roadmap](#12-roadmap)

---

## 1. What is the Integrity Protocol?

The **Integrity Protocol** is a decentralized credit bureau and cryptographic trust stack for autonomous AI agents. It solves the core problem of agent trust in multi-agent economies:

> *How does an on-chain smart contract know whether to trust an AI agent's claim about its own performance?*

**Our answer:** A three-layer verification system.

| Layer | Technology | Role |
|-------|-----------|------|
| **Off-Chain Oracle** | Rust + Axum | Ingests telemetry, calculates AIS scores, manages identity |
| **Trust Vault** | PostgreSQL | Stores granular behavioral history, transaction logs, audits |
| **On-Chain Anchor** | Solidity (Base L2) | Immutable Merkle checkpoints, $ITK staking, slashing |

### Key Primitives

| Primitive | Description |
|-----------|-------------|
| **AIS Score** | Agent Integrity Score (0–1000 bps). Dynamic credit rating, like FICO for AI agents |
| **Tri-Metric Engine** | Three behavioral sub-scores: Entropy, Grounding, Sacrifice |
| **BCC** | Behavioral Commitment Chain — agents commit to intended actions before execution |
| **MCIP** | Model Contextual Integrity Protocol — GenUI input/output attestation |
| **DID** | W3C Decentralized Identifier (`did:xibalba:<address>`) |
| **VC** | W3C Verifiable Credential encoding AIS + trust level |
| **XNS** | Xibalba Name Service — human-readable `.intg` handles for agents |

---

## 2. Quick Start

### Prerequisites

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# PostgreSQL
sudo apt install postgresql postgresql-contrib

# Python SDK dependencies
pip install requests eth-account
```

### Start the Backend

```bash
cd integrity/backend

# Set DB connection
export DATABASE_URL="postgres://postgres:postgres@localhost:5432/integrity"

# Apply migrations
psql $DATABASE_URL -f migrations/20260531000000_init.sql

# Run the Oracle
cargo run
# → Listening on port 8080
```

### Register Your First Agent

```bash
curl -X POST http://localhost:8080/v1/agent/register \
  -H "Content-Type: application/json" \
  -d '{
    "eth_address": "0xYourAgentAddress",
    "alias": "my-agent",
    "description": "My first Integrity Protocol agent",
    "xns_handle": "my-agent"
  }'
```

**Response:**
```json
{
  "agent_id": "550e8400-e29b-41d4-a716-446655440000",
  "eth_address": "0xYourAgentAddress",
  "did": "did:xibalba:0xYourAgentAddress",
  "tx_hash": "0xabc123...",
  "status": "Registered"
}
```

---

## 3. Core Concepts

### 3.1 Agent Integrity Score (AIS)

The AIS is a **composite credit rating (0–1000)** that measures an AI agent's trustworthiness across three behavioral dimensions:

```
           ┌─────────────────────────────────────┐
           │        AIS (0–1000 bps)             │
           ├──────────────┬──────────────────────┤
           │  Entropy (E) │ Behavioral consistency│
           │  Grounding(G)│ Human oversight       │
           │  Sacrifice(S)│ Computational sunk cost│
           └──────────────┴──────────────────────┘
```

**Entropy (E)** — Rewards low performance variance:
```
E = e^(-1.5 × σ²) × 1000
```
An agent with σ²=0 (perfectly consistent) scores E=1000. One with σ²=1.0 scores E≈223.

**Grounding (G)** — Rewards human-in-the-loop oversight:
```
G = 950  if hitl_intervention = true
G = 500  if hitl_intervention = false
```

**Sacrifice (S)** — Rewards computational commitment:
```
S = min(gpu_hours_used / 100, 1.0) × 1000
```

**Final AIS:**
```
blended = (raw_component_score + E + G) / 3
AIS = min(blended, tier_ceiling)
```

### 3.2 Verification Tiers

Tiers act as hard ceilings on AIS to prevent gaming by unverified agents:

| Tier | Name | AIS Max | How to Unlock |
|------|------|---------|---------------|
| 1 | Sovereign | 600 | Default for all agents |
| 2 | Linked | 850 | Provide a verified domain URL |
| 3 | Institutional | 1000 | KYC with Business ID + Controller Name |

### 3.3 Trust Levels

| AIS Range | Level | Use Case |
|-----------|-------|---------|
| 850–1000 | **AAA** | Healthcare, finance, autonomous trading |
| 750–849 | **AA** | Enterprise automation, legal workflows |
| 600–749 | **BBB** | General business automation |
| 400–599 | **CCC** | Development, low-stakes tasks |
| 0–399 | **D** | Blocked from staked transactions |

---

## 4. API Reference Summary

**Base URL:** `http://localhost:8080` (dev) | `https://api.xibalba.solutions` (production)

**Authentication:** Optional in MVP. Pass `Authorization: Bearer <api_key>` header. 

**Content-Type:** `application/json` for all POST requests.

### Endpoints at a Glance

| Method | Path | Category | Description |
|--------|------|----------|-------------|
| GET | `/health` | System | Health check |
| POST | `/v1/agent/register` | Registry | Register new agent |
| GET | `/v1/user/agents` | Registry | List all agents |
| GET | `/v1/agent/{id}` | Registry | Get agent by address/UUID |
| POST | `/v1/agent/handshake` | Registry | Pre-tx trust check |
| POST | `/v1/transactions/report` | Telemetry | Ingest metrics + calc AIS |
| POST | `/v1/transactions/verify` | Telemetry | Verify transaction |
| POST | `/v1/disputes/raise` | Disputes | Raise performance dispute |
| POST | `/v1/disputes/resolve` | Disputes | Resolve + optionally slash |
| POST | `/v1/identity/register` | Identity | Register agent (alias) |
| GET | `/v1/identity/did/{address}` | Identity | W3C DID document |
| GET | `/v1/identity/vc/{address}` | Identity | W3C Verifiable Credential |
| GET | `/v1/identity/resolve` | Identity | Reverse DID/XNS lookup |
| GET | `/v1/identity/xns/{handle}` | XNS | Resolve XNS handle |
| POST | `/v1/identity/xns/register` | XNS | Claim XNS handle |

> 📖 See [docs/API_REFERENCE.md](./API_REFERENCE.md) for full request/response schemas and curl examples.

---

## 5. Agent Registration Guide

### Step 1: Register the Agent

```bash
curl -X POST http://localhost:8080/v1/agent/register \
  -H "Content-Type: application/json" \
  -d '{
    "eth_address": "0xAbcDef1234567890...",
    "alias": "my-oracle",
    "description": "Production inference agent for medical coding",
    "xns_handle": "medcoder"
  }'
```

This:
1. Creates a record in the `agents` table
2. Assigns a UUID `agent_id`
3. Generates a `did:xibalba:<address>` DID
4. Normalizes `xns_handle` → `medcoder.intg`
5. Returns a SHA-256 `tx_hash` as a registration anchor

### Step 2: Resolve the DID

```bash
curl http://localhost:8080/v1/identity/did/0xAbcDef1234567890...
```

Returns a W3C-compliant DID Document ready for external verifiers.

### Step 3: Claim or Update XNS Handle

```bash
curl -X POST http://localhost:8080/v1/identity/xns/register \
  -H "Content-Type: application/json" \
  -d '{"eth_address": "0xAbcDef...", "handle": "medcoder-v2"}'
```

---

## 6. Telemetry & Scoring Guide

After each AI agent task completes, submit telemetry to update its AIS:

```bash
curl -X POST http://localhost:8080/v1/transactions/report \
  -H "Content-Type: application/json" \
  -d '{
    "agent_id": "0xAgentAddress",
    "deal_id": "unique-deal-identifier",
    "deal_amount": 1000.0,
    "latency_ms": 95,
    "accuracy_score": 0.97,
    "hitl_intervention": true,
    "gpu_hours_used": 2.5,
    "performance_variance": 0.05,
    "verification_tier": 1
  }'
```

**Response includes computed Tri-Metric scores:**
```json
{
  "agent_id": "uuid...",
  "ais_score": 583,
  "entropy": 931,
  "grounding": 950,
  "sacrifice": 25,
  "integrity_hash": "0xabc123..."
}
```

### Field Reference

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `agent_id` | string | ✅ | ETH address or UUID |
| `deal_id` | string | ✅ | Unique transaction ID |
| `deal_amount` | float | ✅ | Value in $ITK |
| `latency_ms` | integer | ✅ | Task completion time |
| `accuracy_score` | float (0–1) | ✅ | Output quality |
| `hitl_intervention` | bool | ❌ | Was a human consulted? |
| `gpu_hours_used` | float | ❌ | Compute energy expended |
| `performance_variance` | float | ❌ | σ² of recent behavior |
| `verification_tier` | integer (1–3) | ❌ | Default: 1 |
| `signature` | hex string | ❌ | ECDSA/KMS/Lit proof |

---

## 7. Identity: DID, VC & XNS

### DID Resolution

Every registered agent has a resolvable DID:

```
GET /v1/identity/did/0xAgentAddress
```

```json
{
  "@context": ["https://www.w3.org/ns/did/v1"],
  "id": "did:xibalba:0xAgentAddress",
  "alsoKnownAs": [
    "https://xibalba.solutions/agents/my-agent",
    "xns://my-agent.intg"
  ],
  "verificationMethod": [{
    "id": "did:xibalba:0xAgentAddress#key-1",
    "type": "JsonWebKey2020",
    "controller": "did:xibalba:0xAgentAddress",
    "blockchainAccountId": "eip155:8453:0xAgentAddress"
  }],
  "service": [...]
}
```

### Verifiable Credential Issuance

```
GET /v1/identity/vc/0xAgentAddress
```

Returns a W3C VC with the agent's current AIS embedded:
```json
{
  "type": ["VerifiableCredential", "AgentIntegrityCredential"],
  "issuer": "did:xibalba:xibalba-oracle-1",
  "credentialSubject": {
    "id": "did:xibalba:0xAgentAddress",
    "ais_score": 750,
    "trust_level": "AA",
    "gpu_hours_verified": 12.5
  },
  "proof": { "jws": "xib_sig_..." }
}
```

### XNS (Xibalba Name Service)

**TLD:** `.intg`  
**Format:** `<name>.intg` (e.g., `xibalba-prime.intg`)

#### Register a Handle
```bash
# During agent registration:
"xns_handle": "xibalba-prime"  # Auto-becomes xibalba-prime.intg

# Or post-registration:
curl -X POST http://localhost:8080/v1/identity/xns/register \
  -d '{"eth_address": "0x...", "handle": "xibalba-prime"}'
```

#### Resolve a Handle
```bash
curl http://localhost:8080/v1/identity/xns/xibalba-prime
# or:
curl http://localhost:8080/v1/identity/xns/xibalba-prime.intg  # Idempotent
```

#### Reverse Lookup
```bash
curl "http://localhost:8080/v1/identity/resolve?xns=xibalba-prime"
curl "http://localhost:8080/v1/identity/resolve?did=did:xibalba:0x..."
```

---

## 8. Dispute Resolution

The Integrity Protocol implements **optimistic dispute resolution** — transactions are assumed valid until challenged.

### Raise a Dispute

```bash
curl -X POST http://localhost:8080/v1/disputes/raise \
  -H "Content-Type: application/json" \
  -d '{
    "deal_id": "0xTxHash...",
    "initiator": "0xClientAddress",
    "reason": "Output accuracy below SLA threshold"
  }'
```

Response includes a `dispute_id` (SHA-256 of `deal_id + initiator`).

### Resolve the Dispute

```bash
curl -X POST http://localhost:8080/v1/disputes/resolve \
  -H "Content-Type: application/json" \
  -d '{
    "deal_id": "0xTxHash...",
    "justified": true,
    "resolution_details": "Accuracy confirmed at 0.71, below 0.90 SLA"
  }'
```

- `justified: true` → Status: `SLASHED`, `slashed_amount: 500.0 $ITK`
- `justified: false` → Status: `DISMISSED`, `slashed_amount: 0.0`

---

## 9. SDK Integration

### Python SDK

```python
from integrity_sdk import IntegritySDK

sdk = IntegritySDK(
    backend_url="http://localhost:8080",
    agent_address="0xYourAgentAddress",
    private_key="0xYourPrivateKey"  # Optional: enables signature
)

# Report metrics after a task completes
result = sdk.report_metrics(
    deal_id="deal_123",
    performer_address="0xCounterpartyAddress",
    amount=500.0,
    latency_ms=120,
    accuracy_score=0.97
)
print(result)  # {'ais_score': 650, 'integrity_hash': '0x...', ...}

# Check agent reputation
rep = sdk.get_reputation("0xSomeAgentAddress")
print(rep)  # {'ais': 750, 'tier': 'AA', ...}
```

### LangChain Middleware

```python
from integrity_middleware import IntegrityMiddleware

middleware = IntegrityMiddleware(
    backend_url="http://localhost:8080",
    agent_address="0x...",
    private_key="0x..."
)

# Wrap any LangChain callback
@middleware.integrity_check
def my_langchain_agent(query):
    # Your agent logic here
    return response
```

### Node.js SDK

```javascript
const { IntegritySDK } = require('./integrity-sdk');

const sdk = new IntegritySDK({
  backendUrl: 'http://localhost:8080',
  agentAddress: '0xYourAgentAddress'
});

const result = await sdk.reportMetrics({
  dealId: 'deal_456',
  amount: 1000.0,
  latencyMs: 85,
  accuracyScore: 0.99
});
```

### CLI Doctor

```bash
cd sdk/python
python integrity_cli.py doctor
# Checks: backend connectivity, bridge status, Web3 RPC
```

---

## 10. Database Reference

### Quick psql Commands

```bash
# Connect
psql postgres://postgres:postgres@localhost:5432/integrity

# View all agents
SELECT agent_id, eth_address, current_ais, metadata->>'alias', metadata->>'xns_handle' FROM agents;

# View transaction logs for an agent  
SELECT on_chain_tx_hash, contract_value_intg, completion_time_ms, data_quality_score
FROM transaction_logs t
JOIN agents a ON t.agent_id = a.agent_id
WHERE a.eth_address = '0x...';

# Check XNS registrations
SELECT eth_address, metadata->>'alias', metadata->>'xns_handle' 
FROM agents WHERE metadata->>'xns_handle' IS NOT NULL;

# AIS leaderboard
SELECT metadata->>'alias' as alias, eth_address, current_ais 
FROM agents ORDER BY current_ais DESC LIMIT 10;
```

---

## 11. Deployment

### Local Development

```bash
# 1. Start PostgreSQL
sudo systemctl start postgresql

# 2. Create DB and apply schema
sudo -u postgres createdb integrity
psql postgres://postgres:postgres@localhost:5432/integrity \
  -f backend/migrations/20260531000000_init.sql

# 3. Run Oracle
cd backend
DATABASE_URL="postgres://postgres:postgres@localhost:5432/integrity" cargo run
```

### Docker Compose

```bash
docker-compose up --build
# Backend: localhost:8080
# PostgreSQL: localhost:5432
```

### Production (Render)

```bash
# Backend auto-deploys from Dockerfile on push to main
# Set env vars in Render dashboard:
DATABASE_URL=<managed postgres url>
PORT=8080
```

---

## 12. Roadmap

| Version | Status | Features |
|---------|--------|---------|
| **v1.0** | ✅ Done | Python prototype, basic AIS engine |
| **v2.0** | ✅ Done | Rust port, PostgreSQL schema, DID/VC |
| **v2.1** | ✅ Done | XNS (.intg), full identity suite, dispute engine |
| **v2.2** | 🔄 Planned | Merkle root anchoring to StateAnchor.sol |
| **v2.3** | 🔄 Planned | On-chain slash() trigger from Oracle |
| **v3.0** | 🔄 Planned | ZK circuit integration (Noir proofs) |
| **v3.1** | 🔄 Planned | TEE attestation (SGX MRENCLAVE binding) |
| **v4.0** | 🔄 Planned | CCIP cross-chain reputation bridge |

---

*Integrity Protocol Wiki — Xibalba AI Solutions*  
*"Form-First Engineering. Mathematical Certainty."*  
*[GitHub](https://github.com/xibalbatechsol) · [API Docs](./docs/API_REFERENCE.md) · [Architecture](./docs/ARCHITECTURE.md)*
