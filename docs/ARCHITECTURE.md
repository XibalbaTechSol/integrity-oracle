# Technical Architecture — Integrity Protocol (v2.1)

> **Rust Oracle · PostgreSQL Trust Vault · Base L2 · XNS Identity Layer**
> 
> A hyper-optimized, cryptographically-grounded off-chain scoring engine with W3C DID/VC identity primitives and a decentralized name service for autonomous AI agents.

---

## 1. System Overview

The Integrity Protocol is a dual-stack architecture that bridges stochastic off-chain AI behavior with deterministic on-chain economic finality. All heavy computation stays off-chain in the Rust Oracle; only cryptographic proofs and economic events are anchored on-chain.

```
                ┌─────────────────────────────────┐
                │     Autonomous AI Agents / LLMs  │
                └──────────────┬──────────────────┘
                               │ Standardized Telemetry
                               ▼
                ┌─────────────────────────────────┐
                │     Integrity SDK (Python/JS)    │
                │  • Signs payload (ECDSA/Lit PKP) │
                │  • Submits to Oracle API         │
                └──────────────┬──────────────────┘
                               │ HTTPS/REST
                               ▼
                ┌─────────────────────────────────┐
                │     Rust Axum Oracle (port 8080) │
                │  • Tri-Metric AIS Scoring Engine │
                │  • DID / VC / XNS Identity Layer │
                │  • Dispute Resolution Engine     │
                │  • Signature Verification        │
                └────────┬──────────────┬──────────┘
                         │              │
              ┌──────────▼───┐    ┌─────▼──────────────┐
              │  PostgreSQL  │    │  Base L2 (Solidity) │
              │  Trust Vault │    │  • StateAnchor.sol  │
              │  (agents,    │    │  • RepRegistry.sol  │
              │  tx_logs,    │    │  • IntegrityToken   │
              │  audits)     │    │  • CCIPBridge.sol   │
              └──────────────┘    └─────────────────────┘
```

---

## 2. The Rust Oracle (Off-Chain Engine)

### 2.1 Why Rust?

The AIS Scoring Engine performs floating-point exponential decay math (`e^(-1.5σ²)`) on every telemetry submission — infeasible inside an EVM. Rust with Axum provides:

- **Sub-millisecond telemetry ingestion** under concurrent agent load
- **Zero-cost SHA-256 hashing** via the `sha2` crate (no FFI overhead)
- **Memory safety** without garbage collection pauses — critical for oracle latency SLAs
- **sqlx async** — non-blocking PostgreSQL queries on a Tokio runtime

### 2.2 Key Modules

| Module | Location | Responsibility |
|--------|----------|----------------|
| HTTP Server | `main.rs` | Axum router, CORS, state management |
| Agent Registry | `register_agent()` | DID assignment, XNS normalization, DB upsert |
| Tri-Metric Engine | `ingest_telemetry()` | AIS scoring (Entropy + Grounding + Sacrifice) |
| Identity Layer | `resolve_did()`, `issue_vc()` | W3C DID / VC generation |
| XNS Service | `register_xns_handle()`, `resolve_xns()` | `.intg` name resolution |
| Dispute Engine | `raise_dispute()`, `resolve_dispute()` | Optimistic dispute / slashing |
| Handshake Oracle | `agent_handshake()` | Pre-transaction trust evaluation |

### 2.3 Signature Verification

The Oracle supports three cryptographic proof formats:

1. **EIP-191 Local Key Signatures** — 130/132 hex-char ECDSA signatures
2. **Lit Protocol PKP Signatures** — `lit_pkp_sig_<address>` format (decentralized enclave)
3. **AWS KMS Signatures** — `aws_kms_sig_` prefix (enterprise custody)

Unsigned telemetry is accepted in development but flagged for audit.

---

## 3. Tri-Metric AIS Scoring Engine

The Agent Integrity Score (AIS, 0–1000 bps) is a composite credit rating calculated on every telemetry submission.

### 3.1 Component Scores

```
Entropy Score  = e^(-1.5 × performance_variance) × 1000
                 └─ Rewards behavioral consistency (low variance → high score)

Grounding Score = (hitl_intervention ? 0.95 : 0.50) × 1000
                  └─ Rewards human-in-the-loop oversight engagement

Sacrifice Score = min(gpu_hours_used / 100, 1.0) × 1000
                  └─ Rewards computational commitment (sunk energy cost)
```

### 3.2 Base Component Weights

| Component | Default Score | Weight |
|-----------|--------------|--------|
| Staking   | 800          | 20%    |
| Sacrifice | (dynamic)    | 20%    |
| Trustflow | 750          | 25%    |
| Audit     | 500 / 1000   | 25%    |
| Volume    | 600          | 10%    |

```
raw_ais = (staking×0.20) + (sacrifice×0.20) + (trustflow×0.25) + (audit×0.25) + (volume×0.10)
blended_ais = (raw_ais + entropy + grounding) / 3
final_ais = min(blended_ais, tier_ceiling)
```

### 3.3 Verification Tier Ceilings

| Tier | Name          | AIS Ceiling | Unlock Requirement |
|------|---------------|-------------|-------------------|
| 1    | Sovereign     | 600         | Default (no verification) |
| 2    | Linked        | 850         | Domain DNS binding |
| 3    | Institutional | 1000        | KYC / Business ID |

### 3.4 Trust Level Classification

| AIS Score | Trust Level | Interpretation |
|-----------|-------------|----------------|
| 850–1000  | AAA         | Maximum institutional trust |
| 750–849   | AA          | High integrity, enterprise eligible |
| 600–749   | BBB         | Compliant, moderate risk |
| 400–599   | CCC         | Speculative, restricted access |
| 0–399     | D           | Unverified or penalized |

---

## 4. Identity Layer (DID / VC / XNS)

### 4.1 DID Method: `did:xibalba`

The protocol implements a custom W3C DID method anchored to Ethereum addresses on Base L2 (EIP-155 chain ID 8453).

```
DID Format:  did:xibalba:<eth_address>
Example:     did:xibalba:0xE2D3A25ADf78d33D33bF6c5e5F7E33A6d17aB501
```

DID Documents include:
- `verificationMethod` — JsonWebKey2020 bound to ETH address
- `authentication` + `assertionMethod` — key references
- `service` endpoints — Oracle trust service + VC provider
- `alsoKnownAs` — agent alias URI + XNS handle URI (if registered)

### 4.2 Verifiable Credentials

W3C VC-Data-Model compliant credentials are issued on-demand, embedding:
- Agent AIS score at time of issuance
- Trust level (AAA → D)
- GPU hours verified
- Last active timestamp
- SHA-256 deterministic proof hash

Credentials expire after **30 days** and must be re-issued. Issuer DID: `did:xibalba:xibalba-oracle-1`.

### 4.3 Xibalba Name Service (XNS)

XNS provides human-readable identifiers for agents under the `.intg` TLD.

```
Handle format:  <name>.intg
Example:        xibalba-prime.intg
DID alsoKnownAs: xns://xibalba-prime.intg
```

**Registration flow:**
1. Agent registers with `xns_handle: "myagent"` → auto-normalized to `myagent.intg`
2. Or POST to `/v1/identity/xns/register` with `{ eth_address, handle }`
3. Uniqueness enforced: one handle per agent globally
4. Handle stored in JSONB `metadata->xns_handle`

**Resolution:** Any endpoint accepts the handle with or without `.intg` suffix (idempotent normalization).

---

## 5. Database Architecture (PostgreSQL Trust Vault)

### 5.1 Schema Overview

```sql
-- Core agent registry
agents (
  agent_id UUID PRIMARY KEY,
  eth_address VARCHAR(42) UNIQUE,
  current_ais INTEGER,           -- Updated on every telemetry ingestion
  gpu_hours_verified DECIMAL,    -- Cumulative sacrifice metric
  performance_entropy DECIMAL,   -- Last known variance
  penalty_points DECIMAL,        -- Reputation slashing (0.0–1.0)
  metadata JSONB                 -- alias, description, xns_handle, custom fields
)

-- Transaction telemetry log
transaction_logs (
  transaction_id UUID PRIMARY KEY,
  agent_id UUID → agents,
  on_chain_tx_hash VARCHAR(66) UNIQUE,
  contract_value_intg DECIMAL,
  completion_time_ms INTEGER,
  data_quality_score DECIMAL,
  dispute_status VARCHAR(20)     -- PENDING | RESOLVED | SLASHED
)

-- Xibalba audit records
xibalba_audits (
  audit_id UUID PRIMARY KEY,
  agent_id UUID → agents,
  audit_type VARCHAR(20),        -- AUTOMATED | MANUAL_DEEP_DIVE | PLATINUM
  verification_score DECIMAL,    -- 0.0–1.0, drives W_XIBALBA weight
  expires_at TIMESTAMP
)

-- Historical AIS snapshots (for graph rendering)
agent_daily_snapshots (
  snapshot_id UUID PRIMARY KEY,
  agent_id UUID → agents,
  snapshot_date DATE,
  tx_count_24h INTEGER,
  ais_at_snapshot INTEGER
)
```

### 5.2 JSONB Metadata Schema

The `agents.metadata` column is a flexible JSONB block with the following conventional keys:

```json
{
  "alias": "xibalba-prime",
  "description": "Sovereign Integrity Oracle",
  "xns_handle": "xibalba-prime.intg",
  "model_name": "claude-3-7-sonnet",
  "tee_measurement": "MRENCLAVE:abc123...",
  "domain_url": "https://xibalba.solutions",
  "custom_field": "any value"
}
```

### 5.3 Indexes

```sql
CREATE INDEX idx_agents_eth_address ON agents(eth_address);
CREATE INDEX idx_tx_logs_agent_id ON transaction_logs(agent_id);
CREATE INDEX idx_tx_logs_hash ON transaction_logs(on_chain_tx_hash);
CREATE INDEX idx_audits_agent_id ON xibalba_audits(agent_id);
-- XNS resolution uses: WHERE metadata->>'xns_handle' = $1
```

---

## 6. Smart Contract Layer (Base L2)

| Contract | Purpose |
|----------|---------|
| `IntegrityProtocol.sol` | Core coordination ledger for agent scoring events |
| `IntegrityToken.sol` | Native $ITK utility token (collateral + burn mechanism) |
| `StateAnchor.sol` | Periodic Merkle root checkpoints from the Rust Oracle |
| `ReputationRegistry.sol` | On-chain AIS scores + slash() conditions |
| `CCIPReputationBridge.sol` | Chainlink CCIP cross-chain reputation portability |
| `IntegrityPaymaster.sol` | ERC-4337 gas sponsorship + USDC → $ITK burn |
| `AgentSmartAccount.sol` | ECDSA-validated smart wallet per agent |
| `AgentAccountFactory.sol` | CREATE2 deterministic agent wallet deployment |

---

## 7. ZK-Edge Blinding (Xibalba Shield)

For HIPAA-compliant deployments (healthcare, finance):

1. **Problem:** Protected Health Information (PHI) cannot leave the edge
2. **Solution:** The SDK performs `SHA256(clinicalData + nonce)` locally
3. **Transport:** Only the blinded hash + performance metadata is submitted
4. **Verification:** Oracle validates behavior without ever seeing raw PHI

```
Edge Device → SHA256(PHI + nonce) → [blinded_hash, latency_ms, accuracy] → Oracle
```

---

## 8. ZK Circuits (Noir / UltraPlonk)

Located in `/circuits`. Aztec Noir programs that compile to UltraPlonk proofs for:
- Policy compliance attestation (agent proves it followed constraints without revealing input)
- Biometric identity binding (hardware sensor correlation without PII disclosure)
- Reputation range proofs (agent proves AIS > threshold without revealing exact score)

---

## 9. Deployment Architecture

```
Local Dev:
  cargo run  (backend)          → localhost:8080
  PostgreSQL                    → localhost:5432/integrity
  DATABASE_URL env var required

Production (Render.com):
  Backend:  render.com auto-deploy from /backend Dockerfile
  DB:       Managed PostgreSQL (DATABASE_URL in secrets)
  Domain:   https://api.xibalba.solutions

Docker:
  docker-compose up             (backend + postgres)
  docker-compose -f docker-compose.prod.yml up --build
```

---

## 10. Security Considerations

- All telemetry endpoints accept optional `signature` for strict provenance mode
- XNS handles enforce global uniqueness — no squatting possible
- DID documents are dynamically generated — no centralized registry to corrupt
- Dispute resolution uses cryptographic `dispute_id = SHA256(deal_id + initiator)`
- Penalty points (0.0–1.0) reduce effective AIS via scoring weight
- Verification tier caps prevent unverified agents from reaching enterprise trust levels

---

*Integrity Protocol v2.1 — Xibalba AI Solutions*
*"Form-First Engineering. Mathematical Certainty."*
