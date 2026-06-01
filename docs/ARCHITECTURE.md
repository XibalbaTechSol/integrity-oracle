# Technical Architecture V2: Base L2 + Rust Oracle

## Overview
The V2 rebuild of the Integrity Protocol transitions from a legacy Python monolith to a hyper-optimized, dual-stack architecture. This separation of concerns allows for cheap, lightning-fast telemetry ingestion while maintaining ultimate cryptographic security on Ethereum L1 (Sepolia) and Base L2.

---

## 1. The Separation of Concerns

### The Rust Oracle (Off-Chain Validation Engine)
Calculating exponential decay ($e^{-1.5 \cdot \sigma^2}$) for the Entropy score is computationally expensive and wildly impractical to run entirely inside an EVM smart contract. 
- **The Solution:** We have fully ported the legacy Python (`trust_api.py`) validation service and proprietary reputation database (`schema.sql`) into a unified, high-performance **Rust (Axum + sqlx)** backend.
- The Oracle exposes the `/v1/transactions/verify` and `/v1/agent/register` APIs to ingest raw telemetry.
- It performs the heavy Tri-Metric math in memory, tracks slashing/audits in a highly granular **PostgreSQL** database, and periodically calculates a Merkle Root of the global state.

### The Smart Contracts (Base L2 / Sepolia)
The legacy Solidity contracts (`IntegrityToken`, `ReputationRegistry`, `StateAnchor`) remain the bedrock of the protocol.
- The Rust Oracle submits the calculated Merkle Root to the `StateAnchor.sol` contract.
- If an agent hallucinates, the Oracle calls the on-chain `slash()` function, burning the agent's staked ITK collateral.
- *Note: Our Hardhat environment is explicitly configured to deploy to both Base L2 and the legacy Ethereum Sepolia L1 testnet.*

---

## 2. ZK-Edge Blinding & HIPAA Compliance (Xibalba Shield)
To pilot the protocol in the healthcare sector, we built the **Xibalba Shield**.
- **The Problem:** Protected Health Information (PHI) cannot be sent to an external Oracle or the blockchain.
- **The Edge SDK:** Our TypeScript SDK allows clinical orchestrators to perform a local `SHA-256` hash (`SHA256(clinicalData + nonce)`). 
- Only this blinded "ZK Hash" is transmitted to the Rust Oracle along with the performance metadata (Latency, Accuracy). The protocol verifies the agent's behavior without ever seeing the underlying medical data.

---

## 3. Standardized Telemetry Schema
Agents must submit telemetry to the `/api/telemetry` endpoint using the following JSON schema:

```json
{
  "agent_id": "0xAgentAddress",
  "latency_ms": 240,
  "accuracy_score": 0.98,
  "hitl_intervention": false,
  "gpu_hours_used": 1.5,
  "zk_hash": "0xabc123..."
}
```
*Any deviation from this schema will result in rejected telemetry and a potential drop in the agent's Entropy score.*
