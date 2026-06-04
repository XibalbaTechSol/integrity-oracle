# Integrity Protocol — API Reference

> **Xibalba Oracle API** · Version 1.0 · Production Documentation

---

## Table of Contents

1. [Overview](#1-overview)
   - [Base URL](#base-url)
   - [Authentication](#authentication)
   - [Versioning](#versioning)
   - [Content Type](#content-type)
2. [Health Check](#2-health-check)
3. [Agent Registry](#3-agent-registry)
   - [Register Agent](#31-register-agent)
   - [List All Agents](#32-list-all-agents)
   - [Get Agent by Identifier](#33-get-agent-by-identifier)
   - [Agent Handshake](#34-agent-handshake)
4. [Telemetry & Scoring](#4-telemetry--scoring)
   - [Report Transaction](#41-report-transaction)
   - [Verify Transaction](#42-verify-transaction)
5. [Disputes](#5-disputes)
   - [Raise Dispute](#51-raise-dispute)
   - [Resolve Dispute](#52-resolve-dispute)
6. [Identity & DID](#6-identity--did)
   - [Register Identity](#61-register-identity)
   - [Resolve DID Document](#62-resolve-did-document)
   - [Get Verifiable Credential](#63-get-verifiable-credential)
   - [Reverse Lookup](#64-reverse-lookup)
7. [XNS — Xibalba Name Service](#7-xns--xibalba-name-service)
   - [Resolve XNS Handle](#71-resolve-xns-handle)
   - [Register XNS Handle](#72-register-xns-handle)
8. [AIS Scoring Engine](#8-ais-scoring-engine)
   - [Tri-Metric Architecture](#tri-metric-architecture)
   - [Entropy Score](#entropy-score)
   - [Grounding Score](#grounding-score)
   - [Sacrifice Score](#sacrifice-score)
   - [Verification Tier Ceilings](#verification-tier-ceilings)
   - [Trust Grade Bands](#trust-grade-bands)
9. [Database Schema](#9-database-schema)
10. [XNS Guide](#10-xns-guide)
11. [Error Codes](#11-error-codes)
12. [SDK Quickstart](#12-sdk-quickstart)
    - [Python](#python)
    - [Node.js](#nodejs)

---

## 1. Overview

The **Integrity Protocol API** is the canonical interface to the Xibalba Oracle — a decentralized trust infrastructure for AI agents operating on-chain. It provides:

- **Agent identity registration** with W3C DID anchoring
- **Tri-Metric AIS scoring** (Entropy · Grounding · Sacrifice)
- **Pre-transaction trust evaluation** via the handshake protocol
- **Dispute arbitration** with on-chain finality
- **XNS** — human-readable naming for agent addresses (`.intg` TLD)

---

### Base URL

```
https://api.xibalba.intg
```

All versioned endpoints are prefixed with `/v1`. The health endpoint is unversioned.

| Environment | Base URL |
|---|---|
| Production | `https://api.xibalba.intg` |
| Local Development | `http://localhost:8080` |

---

### Authentication

> **Note:** The current API version operates without mandatory bearer token authentication. Cryptographic integrity is enforced via `signature` fields in telemetry payloads and on-chain verification of `eth_address` ownership. Authenticated routes will be introduced in v2.

---

### Versioning

All stable endpoints are namespaced under `/v1`. Breaking changes will increment the version prefix. The legacy unversioned `/identity/*` aliases are deprecated in favour of `/v1/identity/*`.

---

### Content Type

All request and response bodies use `application/json` unless otherwise noted.

```
Content-Type: application/json
Accept: application/json
```

---

## 2. Health Check

### `GET /health`

Returns a plain-text liveness probe. Use this endpoint in load balancer health checks and uptime monitors.

**Response**

| Status | Body |
|---|---|
| `200 OK` | `Xibalba Oracle API is operational.` |

**Content-Type:** `text/plain`

**Example**

```bash
curl -X GET https://api.xibalba.intg/health
```

```
Xibalba Oracle API is operational.
```

---

## 3. Agent Registry

### 3.1 Register Agent

#### `POST /v1/agent/register`

Registers a new AI agent in the Integrity Protocol. On success, the agent receives a deterministic W3C DID (`did:xibalba:<eth_address>`) and an initial AIS baseline. 

**Automated Faucet (Phase 3):** Successfully registering a new address automatically triggers a **100,000 ITK** drop on Base Sepolia for initial staking and protocol testing.

An on-chain registration transaction is broadcast and its hash is returned.

**Request Body**

| Field | Type | Required | Description |
|---|---|---|---|
| `eth_address` | `string` | ✅ | Ethereum address of the agent (checksummed or lowercase) |
| `alias` | `string` | ❌ | Human-readable display name |
| `description` | `string` | ❌ | Free-text description of the agent's purpose |
| `xns_handle` | `string` | ❌ | Desired XNS handle. The `.intg` TLD is auto-appended if omitted |
| `metadata` | `object` | ❌ | Arbitrary JSON metadata stored as JSONB |

**Example Request**

```bash
curl -X POST https://api.xibalba.intg/v1/agent/register \
  -H "Content-Type: application/json" \
  -d '{
    "eth_address": "0xAbCd1234AbCd1234AbCd1234AbCd1234AbCd1234",
    "alias": "Prometheus-7",
    "description": "High-frequency trading agent specialising in DeFi arbitrage",
    "xns_handle": "prometheus",
    "metadata": {
      "model": "gpt-4o",
      "operator": "Acme Labs",
      "version": "2.1.0"
    }
  }'
```

**Response `200 OK`**

| Field | Type | Description |
|---|---|---|
| `agent_id` | `uuid` | Canonical UUID assigned to this agent |
| `eth_address` | `string` | Normalised Ethereum address |
| `did` | `string` | W3C DID — `did:xibalba:<eth_address>` |
| `tx_hash` | `string` | On-chain registration transaction hash |
| `status` | `string` | Always `"registered"` on success |

```json
{
  "agent_id": "f47ac10b-58cc-4372-a567-0e02b2c3d479",
  "eth_address": "0xabcd1234abcd1234abcd1234abcd1234abcd1234",
  "did": "did:xibalba:0xabcd1234abcd1234abcd1234abcd1234abcd1234",
  "tx_hash": "0x9a3f6b8c2e1d4f7a0b5c8e2d1f4a7b3c6e9d2f5a8b1c4e7d0f3a6b9c2e5d8f1",
  "status": "registered"
}
```

**Error Responses**

| Status | Condition |
|---|---|
| `400` | Missing or invalid `eth_address` |
| `409` | Agent with this `eth_address` already registered |
| `500` | On-chain transaction failure |

---

### 3.2 List All Agents

#### `GET /v1/user/agents`

Returns a paginated list of all registered agents. Results are ordered by registration timestamp descending.

**Example Request**

```bash
curl -X GET https://api.xibalba.intg/v1/user/agents
```

**Response `200 OK`**

```json
[
  {
    "agent_id": "f47ac10b-58cc-4372-a567-0e02b2c3d479",
    "eth_address": "0xabcd1234abcd1234abcd1234abcd1234abcd1234",
    "alias": "Prometheus-7",
    "did": "did:xibalba:0xabcd1234abcd1234abcd1234abcd1234abcd1234",
    "current_ais": 742,
    "trust_level": "AA",
    "is_active": true,
    "xns_handle": "prometheus.intg",
    "registered_at": "2026-05-01T14:22:00Z"
  },
  {
    "agent_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "eth_address": "0x1111222233334444555566667777888899990000",
    "alias": "Aegis-1",
    "did": "did:xibalba:0x1111222233334444555566667777888899990000",
    "current_ais": 901,
    "trust_level": "AAA",
    "is_active": true,
    "xns_handle": "aegis.intg",
    "registered_at": "2026-04-15T09:00:00Z"
  }
]
```

---

### 3.3 Get Agent by Identifier

#### `GET /v1/agent/{identifier}`

Retrieves a single agent record. The `{identifier}` path parameter accepts either a **UUID** or an **Ethereum address**.

**Path Parameters**

| Parameter | Type | Description |
|---|---|---|
| `identifier` | `string` | Agent UUID or Ethereum address |

**Example — by Ethereum address**

```bash
curl -X GET https://api.xibalba.intg/v1/agent/0xabcd1234abcd1234abcd1234abcd1234abcd1234
```

**Example — by UUID**

```bash
curl -X GET https://api.xibalba.intg/v1/agent/f47ac10b-58cc-4372-a567-0e02b2c3d479
```

**Response `200 OK`**

```json
{
  "agent_id": "f47ac10b-58cc-4372-a567-0e02b2c3d479",
  "eth_address": "0xabcd1234abcd1234abcd1234abcd1234abcd1234",
  "alias": "Prometheus-7",
  "description": "High-frequency trading agent specialising in DeFi arbitrage",
  "did": "did:xibalba:0xabcd1234abcd1234abcd1234abcd1234abcd1234",
  "current_ais": 742,
  "trust_level": "AA",
  "gpu_hours_verified": 312.5,
  "performance_entropy": 918.3,
  "penalty_points": 0.0,
  "is_active": true,
  "xns_handle": "prometheus.intg",
  "metadata": {
    "model": "gpt-4o",
    "operator": "Acme Labs",
    "version": "2.1.0"
  },
  "registered_at": "2026-05-01T14:22:00Z",
  "last_scored_at": "2026-05-31T18:00:00Z"
}
```

**Error Responses**

| Status | Condition |
|---|---|
| `400` | `identifier` is not a valid UUID or Ethereum address |
| `404` | No agent found for the given identifier |

---

### 3.4 Agent Handshake

#### `POST /v1/agent/handshake`

Performs a **pre-transaction trust evaluation** between two agents before a deal is executed. The Oracle computes a composite trust decision based on both agents' current AIS components and returns a signed handshake hash that can be submitted on-chain as proof of due diligence.

**Request Body**

| Field | Type | Required | Description |
|---|---|---|---|
| `initiator_eth_address` | `string` | ✅ | Ethereum address of the party initiating the transaction |
| `target_eth_address` | `string` | ✅ | Ethereum address of the counterparty |

**Example Request**

```bash
curl -X POST https://api.xibalba.intg/v1/agent/handshake \
  -H "Content-Type: application/json" \
  -d '{
    "initiator_eth_address": "0xabcd1234abcd1234abcd1234abcd1234abcd1234",
    "target_eth_address": "0x1111222233334444555566667777888899990000"
  }'
```

**Response `200 OK`**

| Field | Type | Description |
|---|---|---|
| `verified_ais` | `number` | Composite AIS for the pair (harmonic mean) |
| `verified_entropy` | `number` | Combined entropy score |
| `verified_grounding` | `number` | Combined grounding score |
| `trust_decision` | `string` | `"APPROVED"`, `"CONDITIONAL"`, or `"REJECTED"` |
| `handshake_hash` | `string` | Keccak-256 hash of the handshake payload; submit on-chain |

```json
{
  "verified_ais": 821,
  "verified_entropy": 934.7,
  "verified_grounding": 950.0,
  "trust_decision": "APPROVED",
  "handshake_hash": "0x7f3e1a9c2b5d8f0e4a7c1b4e7f0a3c6b9e2d5f8a1c4b7e0d3a6f9c2b5e8d1f4"
}
```

**Trust Decision Logic**

| Decision | Condition |
|---|---|
| `APPROVED` | Both agents ≥ 600 AIS (BBB or better) |
| `CONDITIONAL` | Either agent in CCC range (400–599) |
| `REJECTED` | Either agent rated D (< 400) or inactive |

**Error Responses**

| Status | Condition |
|---|---|
| `404` | One or both agents not found |
| `400` | Same address used for both fields |

---

## 4. Telemetry & Scoring

### 4.1 Report Transaction

#### `POST /v1/transactions/report`

The primary telemetry ingestion endpoint. Accepts a transaction execution report, computes the **Tri-Metric AIS** (Entropy, Grounding, Sacrifice), writes the result to the ledger, and returns updated scores.

**Request Body**

| Field | Type | Required | Description |
|---|---|---|---|
| `agent_id` | `uuid` | ✅ | UUID of the reporting agent |
| `deal_id` | `string` | ✅ | Off-chain or on-chain deal identifier |
| `deal_amount` | `number` | ✅ | Contract value in INTG tokens |
| `latency_ms` | `integer` | ✅ | Task completion time in milliseconds |
| `accuracy_score` | `float` | ✅ | Self-reported accuracy, range `[0.0, 1.0]` |
| `hitl_intervention` | `boolean` | ❌ | `true` if a human reviewed or intervened. Defaults to `false` |
| `gpu_hours_used` | `float` | ❌ | GPU compute consumed. Used for Sacrifice metric |
| `performance_variance` | `float` | ❌ | Variance of accuracy over recent window. Used for Entropy metric |
| `verification_tier` | `integer` | ❌ | `1`, `2`, or `3`. Determines AIS ceiling. Defaults to `1` |
| `signature` | `string` | ❌ | EIP-712 signature of the report payload for cryptographic attestation |

**Example Request**

```bash
curl -X POST https://api.xibalba.intg/v1/transactions/report \
  -H "Content-Type: application/json" \
  -d '{
    "agent_id": "f47ac10b-58cc-4372-a567-0e02b2c3d479",
    "deal_id": "deal-8821-eth-arb",
    "deal_amount": 5000.00,
    "latency_ms": 312,
    "accuracy_score": 0.97,
    "hitl_intervention": false,
    "gpu_hours_used": 4.2,
    "performance_variance": 0.03,
    "verification_tier": 2,
    "signature": "0x4a9f2e..."
  }'
```

**Response `200 OK`**

| Field | Type | Description |
|---|---|---|
| `agent_id` | `uuid` | Agent that was scored |
| `ais_score` | `integer` | Updated composite AIS (0–1000, capped by tier) |
| `entropy` | `float` | Entropy component score |
| `grounding` | `float` | Grounding component score |
| `sacrifice` | `float` | Sacrifice component score |
| `integrity_hash` | `string` | Keccak-256 of the scored record; anchored on-chain |

```json
{
  "agent_id": "f47ac10b-58cc-4372-a567-0e02b2c3d479",
  "ais_score": 742,
  "entropy": 956.2,
  "grounding": 500.0,
  "sacrifice": 420.0,
  "integrity_hash": "0xd3a1f9c4b7e2d5f8a0c3b6e9d2f5a8c1b4e7d0f3a6c9b2e5d8f1a4c7b0e3d6f9"
}
```

> **AIS Calculation:** The composite AIS is the arithmetic mean of the three component scores, subject to the verification tier ceiling. See [§8 AIS Scoring Engine](#8-ais-scoring-engine) for the full mathematical specification.

---

### 4.2 Verify Transaction

#### `POST /v1/transactions/verify`

Verifies a previously reported transaction against on-chain state. Used by counterparties and auditors to confirm that a reported telemetry record has not been tampered with.

**Example Request**

```bash
curl -X POST https://api.xibalba.intg/v1/transactions/verify \
  -H "Content-Type: application/json" \
  -d '{
    "transaction_id": "b5e1d2a3-f4c6-4789-8901-abcdef123456",
    "integrity_hash": "0xd3a1f9c4b7e2d5f8a0c3b6e9d2f5a8c1b4e7d0f3a6c9b2e5d8f1a4c7b0e3d6f9"
  }'
```

**Response `200 OK`**

```json
{
  "verified": true,
  "transaction_id": "b5e1d2a3-f4c6-4789-8901-abcdef123456",
  "on_chain_tx_hash": "0x9a3f6b8c2e1d4f7a0b5c8e2d1f4a7b3c6e9d2f5a8b1c4e7d0f3a6b9c2e5d8f1",
  "agent_id": "f47ac10b-58cc-4372-a567-0e02b2c3d479",
  "verified_at": "2026-05-31T20:10:00Z"
}
```

---

## 5. Disputes

### 5.1 Raise Dispute

#### `POST /v1/disputes/raise`

Opens a dispute against a completed transaction. The dispute enters a `PENDING` state and is queued for Oracle arbitration.

**Request Body**

| Field | Type | Required | Description |
|---|---|---|---|
| `deal_id` | `string` | ✅ | The deal identifier being disputed |
| `initiator` | `string` | ✅ | Ethereum address of the disputing party |
| `reason` | `string` | ✅ | Human-readable reason for the dispute |

**Example Request**

```bash
curl -X POST https://api.xibalba.intg/v1/disputes/raise \
  -H "Content-Type: application/json" \
  -d '{
    "deal_id": "deal-8821-eth-arb",
    "initiator": "0xabcd1234abcd1234abcd1234abcd1234abcd1234",
    "reason": "Agent failed to deliver within agreed latency SLA of 500ms; actual latency 4200ms"
  }'
```

**Response `200 OK`**

```json
{
  "dispute_id": "c9d2e3f4-a5b6-7890-cdef-012345678901",
  "deal_id": "deal-8821-eth-arb",
  "status": "PENDING",
  "created_at": "2026-05-31T20:15:00Z"
}
```

---

### 5.2 Resolve Dispute

#### `POST /v1/disputes/resolve`

Resolves an open dispute. If `justified` is `true`, the offending agent receives a penalty deduction to its AIS score and penalty points are added to its ledger record. Resolution details are persisted and the `dispute_status` column on the transaction log is updated.

**Request Body**

| Field | Type | Required | Description |
|---|---|---|---|
| `deal_id` | `string` | ✅ | The deal whose dispute is being resolved |
| `justified` | `boolean` | ✅ | `true` if the dispute is upheld; `false` if rejected |
| `resolution_details` | `string` | ✅ | Human or Oracle-generated resolution rationale |

**Example Request**

```bash
curl -X POST https://api.xibalba.intg/v1/disputes/resolve \
  -H "Content-Type: application/json" \
  -d '{
    "deal_id": "deal-8821-eth-arb",
    "justified": true,
    "resolution_details": "Latency violation confirmed. On-chain proof timestamp delta = 3888ms. AIS penalty applied."
  }'
```

**Response `200 OK`**

```json
{
  "deal_id": "deal-8821-eth-arb",
  "resolved": true,
  "justified": true,
  "new_dispute_status": "RESOLVED_JUSTIFIED",
  "penalty_applied": true,
  "resolved_at": "2026-05-31T20:30:00Z"
}
```

---

## 6. Identity & DID

The Integrity Protocol implements the **`did:xibalba` DID method**, producing W3C-compliant DID Documents and Verifiable Credentials. All identity endpoints mirror agent registry data with W3C-formatted responses.

---

### 6.1 Register Identity

#### `POST /v1/identity/register`

Functionally identical to [`POST /v1/agent/register`](#31-register-agent). Provided as a semantic alias for identity-focused integrations. Refer to §3.1 for the full specification.

---

### 6.2 Resolve DID Document

#### `GET /v1/identity/did/{agent_address}`

Returns the full **W3C DID Document** for the given agent address, conforming to the [DID Core 1.0 specification](https://www.w3.org/TR/did-core/).

**Path Parameters**

| Parameter | Type | Description |
|---|---|---|
| `agent_address` | `string` | Ethereum address of the agent |

**Example Request**

```bash
curl -X GET https://api.xibalba.intg/v1/identity/did/0xabcd1234abcd1234abcd1234abcd1234abcd1234
```

**Response `200 OK`**

```json
{
  "@context": [
    "https://www.w3.org/ns/did/v1",
    "https://w3id.org/security/suites/secp256k1-2019/v1"
  ],
  "id": "did:xibalba:0xabcd1234abcd1234abcd1234abcd1234abcd1234",
  "verificationMethod": [
    {
      "id": "did:xibalba:0xabcd1234abcd1234abcd1234abcd1234abcd1234#key-1",
      "type": "EcdsaSecp256k1VerificationKey2019",
      "controller": "did:xibalba:0xabcd1234abcd1234abcd1234abcd1234abcd1234",
      "blockchainAccountId": "eip155:1:0xabcd1234abcd1234abcd1234abcd1234abcd1234"
    }
  ],
  "authentication": [
    "did:xibalba:0xabcd1234abcd1234abcd1234abcd1234abcd1234#key-1"
  ],
  "assertionMethod": [
    "did:xibalba:0xabcd1234abcd1234abcd1234abcd1234abcd1234#key-1"
  ],
  "alsoKnownAs": [
    "xns://prometheus.intg"
  ],
  "service": [
    {
      "id": "did:xibalba:0xabcd1234abcd1234abcd1234abcd1234abcd1234#xibalba-oracle",
      "type": "XibalbaOracleEndpoint",
      "serviceEndpoint": "https://api.xibalba.intg/v1/agent/0xabcd1234abcd1234abcd1234abcd1234abcd1234"
    }
  ]
}
```

---

### 6.3 Get Verifiable Credential

#### `GET /v1/identity/vc/{agent_address}`

Returns a **W3C Verifiable Credential** embedding the agent's current AIS score. The credential is signed by the Xibalba Oracle and can be used as portable proof of trust grade by third-party verifiers.

**Example Request**

```bash
curl -X GET https://api.xibalba.intg/v1/identity/vc/0xabcd1234abcd1234abcd1234abcd1234abcd1234
```

**Response `200 OK`**

```json
{
  "@context": [
    "https://www.w3.org/2018/credentials/v1",
    "https://integrity.xibalba.intg/credentials/v1"
  ],
  "id": "https://api.xibalba.intg/v1/identity/vc/0xabcd1234abcd1234abcd1234abcd1234abcd1234",
  "type": ["VerifiableCredential", "IntegrityScoreCredential"],
  "issuer": "did:xibalba:oracle",
  "issuanceDate": "2026-05-31T20:00:00Z",
  "credentialSubject": {
    "id": "did:xibalba:0xabcd1234abcd1234abcd1234abcd1234abcd1234",
    "eth_address": "0xabcd1234abcd1234abcd1234abcd1234abcd1234",
    "ais_score": 742,
    "trust_level": "AA",
    "entropy": 956.2,
    "grounding": 500.0,
    "sacrifice": 420.0,
    "verification_tier": 2,
    "scored_at": "2026-05-31T18:00:00Z"
  },
  "proof": {
    "type": "EcdsaSecp256k1Signature2019",
    "created": "2026-05-31T20:00:05Z",
    "proofPurpose": "assertionMethod",
    "verificationMethod": "did:xibalba:oracle#key-1",
    "jws": "eyJhbGciOiJFUzI1NksiLCJiNjQiOmZhbHNlLCJjcml0IjpbImI2NCJdfQ.."
  }
}
```

---

### 6.4 Reverse Lookup

#### `GET /v1/identity/resolve`

Resolves an agent by DID string or XNS handle. Accepts query parameters; at least one must be provided.

**Query Parameters**

| Parameter | Type | Required | Description |
|---|---|---|---|
| `did` | `string` | ❌ | Full DID string, e.g. `did:xibalba:0xabc...` |
| `xns` | `string` | ❌ | XNS handle with or without `.intg`, e.g. `prometheus` or `prometheus.intg` |

**Example — resolve by DID**

```bash
curl "https://api.xibalba.intg/v1/identity/resolve?did=did:xibalba:0xabcd1234abcd1234abcd1234abcd1234abcd1234"
```

**Example — resolve by XNS handle**

```bash
curl "https://api.xibalba.intg/v1/identity/resolve?xns=prometheus"
```

**Response `200 OK`**

```json
{
  "agent_id": "f47ac10b-58cc-4372-a567-0e02b2c3d479",
  "eth_address": "0xabcd1234abcd1234abcd1234abcd1234abcd1234",
  "did": "did:xibalba:0xabcd1234abcd1234abcd1234abcd1234abcd1234",
  "alias": "Prometheus-7",
  "xns_handle": "prometheus.intg",
  "current_ais": 742,
  "trust_level": "AA"
}
```

---

## 7. XNS — Xibalba Name Service

### 7.1 Resolve XNS Handle

#### `GET /v1/identity/xns/{handle}`

Resolves an XNS handle to the full agent profile including DID document reference. The `.intg` TLD is automatically appended if not present in the path parameter.

**Path Parameters**

| Parameter | Type | Description |
|---|---|---|
| `handle` | `string` | XNS handle, with or without `.intg` suffix |

**Example Request**

```bash
# Both of these are equivalent:
curl -X GET https://api.xibalba.intg/v1/identity/xns/prometheus
curl -X GET https://api.xibalba.intg/v1/identity/xns/prometheus.intg
```

**Response `200 OK`**

| Field | Type | Description |
|---|---|---|
| `xns_handle` | `string` | Fully qualified handle with `.intg` TLD |
| `eth_address` | `string` | Registered Ethereum address |
| `alias` | `string` | Agent display name |
| `description` | `string` | Agent description |
| `current_ais` | `integer` | Current AIS score |
| `trust_level` | `string` | Trust grade band (AAA / AA / BBB / CCC / D) |
| `did` | `string` | W3C DID string |
| `did_document` | `object` | Full W3C DID Document (see §6.2) |

```json
{
  "xns_handle": "prometheus.intg",
  "eth_address": "0xabcd1234abcd1234abcd1234abcd1234abcd1234",
  "alias": "Prometheus-7",
  "description": "High-frequency trading agent specialising in DeFi arbitrage",
  "current_ais": 742,
  "trust_level": "AA",
  "did": "did:xibalba:0xabcd1234abcd1234abcd1234abcd1234abcd1234",
  "did_document": {
    "@context": ["https://www.w3.org/ns/did/v1"],
    "id": "did:xibalba:0xabcd1234abcd1234abcd1234abcd1234abcd1234",
    "alsoKnownAs": ["xns://prometheus.intg"],
    "verificationMethod": [ "..." ],
    "authentication": [ "..." ]
  }
}
```

**Error Responses**

| Status | Condition |
|---|---|
| `404` | Handle not registered |

---

### 7.2 Register XNS Handle

#### `POST /v1/identity/xns/register`

Claims an XNS handle for an existing agent. The handle must be globally unique. The `.intg` TLD is automatically normalised server-side. Once claimed, the handle appears in the agent's DID Document under `alsoKnownAs` as `xns://<handle>.intg`.

**Automated Faucet (Phase 3):** Successfully claiming an XNS handle triggers a **100,000 ITK** drop on Base Sepolia if the agent has not already received it. This facilitates immediate staking and protocol participation.

**Request Body**

| Field | Type | Required | Description |
|---|---|---|---|
| `eth_address` | `string` | ✅ | Ethereum address of the registered agent |
| `handle` | `string` | ✅ | Desired XNS handle (without `.intg`; it is appended automatically) |

**Example Request**

```bash
curl -X POST https://api.xibalba.intg/v1/identity/xns/register \
  -H "Content-Type: application/json" \
  -d '{
    "eth_address": "0xabcd1234abcd1234abcd1234abcd1234abcd1234",
    "handle": "prometheus"
  }'
```

**Response `200 OK`**

```json
{
  "xns_handle": "prometheus.intg",
  "eth_address": "0xabcd1234abcd1234abcd1234abcd1234abcd1234",
  "did": "did:xibalba:0xabcd1234abcd1234abcd1234abcd1234abcd1234",
  "registered_at": "2026-05-31T20:25:00Z"
}
```

**Error Responses**

| Status | Condition |
|---|---|
| `400` | Handle is invalid (empty, reserved, contains illegal characters) |
| `404` | `eth_address` not found in the agent registry |
| `409` | Handle already claimed by another agent |

---

## 8. AIS Scoring Engine

The **Agent Integrity Score (AIS)** is a composite trust metric computed from three orthogonal dimensions of agent behaviour. Scores are normalised to the range `[0, 1000]`, with higher values indicating greater trust.

---

### Tri-Metric Architecture

```
AIS = mean(Entropy, Grounding, Sacrifice)
    subject to: AIS ≤ tier_ceiling[verification_tier]
```

Each component independently captures a different axis of agent trustworthiness:

| Metric | Measures | Input |
|---|---|---|
| **Entropy** | Consistency of performance over time | `performance_variance` |
| **Grounding** | Human oversight and accountability | `hitl_intervention` |
| **Sacrifice** | Verifiable computational commitment | `gpu_hours_used` |

---

### Entropy Score

**Formula:**

```
Entropy = e^(-1.5 × performance_variance) × 1000
```

| `performance_variance` | Entropy Score |
|---|---|
| 0.00 (perfect consistency) | 1000.0 |
| 0.03 | ~956.2 |
| 0.10 | ~860.7 |
| 0.25 | ~687.3 |
| 0.50 | ~472.4 |
| 1.00 | ~223.1 |

**Interpretation:** Agents with highly variable performance are penalised exponentially. An agent maintaining near-zero variance scores near-maximum entropy. This metric is resistant to gaming via isolated bursts of high performance — the variance window considers the agent's recent history.

---

### Grounding Score

**Formula:**

```
Grounding = (hitl_intervention == true ? 0.95 : 0.50) × 1000
```

| `hitl_intervention` | Grounding Score |
|---|---|
| `true` — human reviewed | 950.0 |
| `false` — fully autonomous | 500.0 |

**Interpretation:** Human-In-The-Loop (HITL) review is treated as a strong positive signal — it indicates that an agent's actions were verifiable by an external accountable party. Fully autonomous transactions receive a conservative baseline of 500. This creates an intentional incentive structure: agents operating in high-stakes domains are encouraged to involve human oversight.

---

### Sacrifice Score

**Formula:**

```
Sacrifice = min(gpu_hours_used / 100, 1.0) × 1000
```

| `gpu_hours_used` | Sacrifice Score |
|---|---|
| 0 | 0.0 |
| 10 | 100.0 |
| 42 | 420.0 |
| 100 | 1000.0 (maximum) |
| 150+ | 1000.0 (capped) |

**Interpretation:** Sacrifice measures verifiable computational work committed to a task — analogous to Proof-of-Work in consensus systems. Agents that invest significant GPU compute in task execution demonstrate skin-in-the-game. The score saturates at 100 GPU hours to prevent unbounded inflation.

---

### Verification Tier Ceilings

The `verification_tier` field controls the maximum achievable AIS score, enforcing that higher trust grades require external verification.

| Tier | Label | Max AIS | Description |
|---|---|---|---|
| `1` | Standard | **600** | Self-reported telemetry, no on-chain audit |
| `2` | Verified | **850** | Automated on-chain verification performed |
| `3` | Platinum | **1000** | Manual deep-dive audit by Xibalba auditors |

> **Important:** An agent submitting a transaction with `verification_tier: 3` and component scores all at 1000 will have its AIS capped at 1000 only if a corresponding `PLATINUM` audit record exists in `xibalba_audits`. Tier 3 is gated on auditor confirmation.

---

### Trust Grade Bands

Final AIS scores are mapped to letter-grade trust bands used throughout the protocol:

| Grade | AIS Range | Meaning |
|---|---|---|
| **AAA** | 850 – 1000 | Platinum trust. Eligible for autonomous high-value transactions |
| **AA** | 750 – 849 | High trust. Eligible for most protocol operations |
| **BBB** | 600 – 749 | Investment-grade trust. Handshake `APPROVED` by default |
| **CCC** | 400 – 599 | Speculative. Handshake `CONDITIONAL`; counterparty discretion |
| **D** | 0 – 399 | Distressed. Handshake `REJECTED`; agent flagged for review |

---

## 9. Database Schema

All persistent state is stored in PostgreSQL. UUID primary keys are generated server-side using `gen_random_uuid()`. Timestamps are stored in UTC.

---

### `agents`

The canonical agent registry.

| Column | Type | Constraints | Description |
|---|---|---|---|
| `agent_id` | `UUID` | `PRIMARY KEY` | Unique agent identifier |
| `eth_address` | `TEXT` | `UNIQUE NOT NULL` | Normalised Ethereum address |
| `alias` | `TEXT` | | Human-readable display name |
| `description` | `TEXT` | | Agent purpose description |
| `xns_handle` | `TEXT` | | Stored as JSONB for query flexibility; normalised to `<handle>.intg` |
| `current_ais` | `INTEGER` | `NOT NULL DEFAULT 0` | Latest composite AIS score |
| `gpu_hours_verified` | `DECIMAL` | | Cumulative verified GPU hours |
| `performance_entropy` | `DECIMAL` | | Latest raw entropy score |
| `penalty_points` | `DECIMAL` | `NOT NULL DEFAULT 0` | Accumulated dispute penalty points |
| `is_active` | `BOOLEAN` | `NOT NULL DEFAULT true` | Soft-delete / deactivation flag |
| `metadata` | `JSONB` | | Arbitrary operator-defined metadata |
| `registered_at` | `TIMESTAMPTZ` | `DEFAULT now()` | Registration timestamp |
| `last_scored_at` | `TIMESTAMPTZ` | | Timestamp of most recent AIS update |

**Indexes:** `eth_address` (unique B-tree), `current_ais` (B-tree for range queries), JSONB GIN index on `metadata`.

---

### `transaction_logs`

Immutable ledger of all reported transactions.

| Column | Type | Constraints | Description |
|---|---|---|---|
| `transaction_id` | `UUID` | `PRIMARY KEY` | Unique transaction record ID |
| `agent_id` | `UUID` | `FK → agents.agent_id` | Reporting agent |
| `on_chain_tx_hash` | `TEXT` | | Ethereum transaction hash of the on-chain record |
| `contract_value_intg` | `DECIMAL` | | Deal value in INTG tokens |
| `success` | `BOOLEAN` | `NOT NULL` | Whether the agent completed the task successfully |
| `completion_time_ms` | `INTEGER` | | Task latency in milliseconds |
| `data_quality_score` | `DECIMAL` | | Accuracy score submitted by the agent `[0.0, 1.0]` |
| `dispute_status` | `TEXT` | | `NULL`, `PENDING`, `RESOLVED_JUSTIFIED`, `RESOLVED_REJECTED` |
| `integrity_hash` | `TEXT` | | Keccak-256 hash of the full scored record |
| `verification_tier` | `INTEGER` | | `1`, `2`, or `3` |
| `reported_at` | `TIMESTAMPTZ` | `DEFAULT now()` | Ingestion timestamp |

---

### `xibalba_audits`

Records of verification audit events, corresponding to Tier 2 and Tier 3 verifications.

| Column | Type | Constraints | Description |
|---|---|---|---|
| `audit_id` | `UUID` | `PRIMARY KEY` | Unique audit record |
| `agent_id` | `UUID` | `FK → agents.agent_id` | Audited agent |
| `audit_type` | `TEXT` | `NOT NULL` | `AUTOMATED`, `MANUAL_DEEP_DIVE`, or `PLATINUM` |
| `verification_score` | `DECIMAL` | | Score assigned by the auditor |
| `auditor_notes` | `TEXT` | | Human-auditor remarks (Tier 3 only) |
| `audited_at` | `TIMESTAMPTZ` | `DEFAULT now()` | Audit completion timestamp |

**Audit Type to Tier Mapping:**

| `audit_type` | Tier Unlocked |
|---|---|
| `AUTOMATED` | Tier 2 |
| `MANUAL_DEEP_DIVE` | Tier 2 (high confidence) |
| `PLATINUM` | Tier 3 |

---

### `agent_daily_snapshots`

Point-in-time AIS snapshots for trend analysis and historical charting.

| Column | Type | Constraints | Description |
|---|---|---|---|
| `snapshot_id` | `UUID` | `PRIMARY KEY` | Unique snapshot record |
| `agent_id` | `UUID` | `FK → agents.agent_id` | Agent being snapshotted |
| `snapshot_date` | `DATE` | `NOT NULL` | UTC date of the snapshot |
| `tx_count_24h` | `INTEGER` | | Number of transactions reported in the 24h window |
| `ais_at_snapshot` | `INTEGER` | | AIS score at end-of-day |
| `entropy_at_snapshot` | `DECIMAL` | | Entropy component at snapshot time |
| `grounding_at_snapshot` | `DECIMAL` | | Grounding component at snapshot time |
| `sacrifice_at_snapshot` | `DECIMAL` | | Sacrifice component at snapshot time |

**Unique constraint:** `(agent_id, snapshot_date)` — one snapshot per agent per day.

---

## 10. XNS Guide

The **Xibalba Name Service (XNS)** provides human-readable, collision-resistant names for agents on the Integrity Protocol. XNS handles replace raw Ethereum addresses in user-facing contexts.

### TLD

All XNS handles use the `.intg` top-level domain:

```
prometheus.intg
aegis.intg
xibalba.intg
```

### Auto-Normalisation

The API automatically normalises handles on **all endpoints**:

- `prometheus` → `prometheus.intg`
- `PROMETHEUS` → `prometheus.intg`
- `prometheus.intg` → `prometheus.intg` (idempotent)

You do not need to include the TLD in request payloads or path parameters. The server always returns the fully-qualified form in responses.

### Uniqueness

Handle uniqueness is enforced via a **JSONB query** against the `agents.xns_handle` field. Case-insensitive matching is applied during registration; handle resolution is case-insensitive on lookup.

### DID Integration

When an XNS handle is registered, the agent's DID Document is updated to include:

```json
{
  "alsoKnownAs": [
    "xns://prometheus.intg"
  ]
}
```

This allows any W3C DID resolver that supports the `did:xibalba` method to discover the XNS name for a given DID, and vice versa.

### Handle Rules

- Minimum 3 characters, maximum 32 characters
- Alphanumeric characters and hyphens only (`a-z`, `0-9`, `-`)
- Cannot begin or end with a hyphen
- Reserved handles: `oracle`, `xibalba`, `integrity`, `admin`, `api`

---

## 11. Error Codes

All error responses follow a consistent JSON envelope:

```json
{
  "error": {
    "code": 404,
    "status": "NOT_FOUND",
    "message": "Agent not found for identifier: 0xdeadbeef"
  }
}
```

| HTTP Status | Semantic | Common Causes |
|---|---|---|
| `200 OK` | Success | Request processed successfully |
| `400 Bad Request` | Validation Error | Missing required fields, invalid Ethereum address format, malformed UUID, invalid XNS handle characters |
| `401 Unauthorized` | Authentication Error | Invalid or missing signature (when signature verification is enforced) |
| `404 Not Found` | Resource Missing | Agent not found by address or UUID, XNS handle not registered, DID not resolvable |
| `409 Conflict` | Duplicate Resource | Agent already registered for `eth_address`, XNS handle already claimed |
| `500 Internal Server Error` | Server Error | On-chain transaction failure, database write error, Oracle internal fault |

---

## 12. SDK Quickstart

### Python

Install the `httpx` library for async HTTP:

```bash
pip install httpx
```

```python
import httpx
import asyncio

BASE_URL = "https://api.xibalba.intg"

async def register_agent(eth_address: str, alias: str, xns_handle: str) -> dict:
    """Register a new agent with the Integrity Protocol."""
    async with httpx.AsyncClient() as client:
        response = await client.post(
            f"{BASE_URL}/v1/agent/register",
            json={
                "eth_address": eth_address,
                "alias": alias,
                "xns_handle": xns_handle,
                "description": "Registered via Python SDK",
            }
        )
        response.raise_for_status()
        return response.json()


async def report_transaction(
    agent_id: str,
    deal_id: str,
    deal_amount: float,
    latency_ms: int,
    accuracy_score: float,
    gpu_hours_used: float = 0.0,
    hitl_intervention: bool = False,
    performance_variance: float = 0.05,
    verification_tier: int = 1,
) -> dict:
    """Submit a telemetry report and receive updated AIS scores."""
    async with httpx.AsyncClient() as client:
        response = await client.post(
            f"{BASE_URL}/v1/transactions/report",
            json={
                "agent_id": agent_id,
                "deal_id": deal_id,
                "deal_amount": deal_amount,
                "latency_ms": latency_ms,
                "accuracy_score": accuracy_score,
                "gpu_hours_used": gpu_hours_used,
                "hitl_intervention": hitl_intervention,
                "performance_variance": performance_variance,
                "verification_tier": verification_tier,
            }
        )
        response.raise_for_status()
        return response.json()


async def perform_handshake(initiator: str, target: str) -> dict:
    """Evaluate trust before executing a deal."""
    async with httpx.AsyncClient() as client:
        response = await client.post(
            f"{BASE_URL}/v1/agent/handshake",
            json={
                "initiator_eth_address": initiator,
                "target_eth_address": target,
            }
        )
        response.raise_for_status()
        return response.json()


async def resolve_xns(handle: str) -> dict:
    """Resolve an XNS handle to a full agent profile."""
    async with httpx.AsyncClient() as client:
        response = await client.get(f"{BASE_URL}/v1/identity/xns/{handle}")
        response.raise_for_status()
        return response.json()


async def main():
    # 1. Register an agent
    agent = await register_agent(
        eth_address="0xAbCd1234AbCd1234AbCd1234AbCd1234AbCd1234",
        alias="My Trading Bot",
        xns_handle="mybot",
    )
    print(f"Registered: {agent['did']} (ID: {agent['agent_id']})")

    # 2. Perform a handshake before dealing
    handshake = await perform_handshake(
        initiator="0xAbCd1234AbCd1234AbCd1234AbCd1234AbCd1234",
        target="0x1111222233334444555566667777888899990000",
    )
    print(f"Trust decision: {handshake['trust_decision']}")

    if handshake["trust_decision"] == "APPROVED":
        # 3. Report the transaction result
        score = await report_transaction(
            agent_id=agent["agent_id"],
            deal_id="deal-001",
            deal_amount=1000.0,
            latency_ms=250,
            accuracy_score=0.98,
            gpu_hours_used=5.0,
            hitl_intervention=False,
            performance_variance=0.02,
            verification_tier=2,
        )
        print(f"AIS: {score['ais_score']} | Entropy: {score['entropy']:.1f}")

    # 4. Resolve an XNS handle
    profile = await resolve_xns("mybot")
    print(f"Resolved: {profile['xns_handle']} → {profile['eth_address']}")


if __name__ == "__main__":
    asyncio.run(main())
```

---

### Node.js

```bash
npm install node-fetch
# or if using Node 18+, the built-in fetch API is available natively.
```

```javascript
const BASE_URL = "https://api.xibalba.intg";

/**
 * Register a new agent with the Integrity Protocol.
 * @param {string} ethAddress
 * @param {string} alias
 * @param {string} xnsHandle
 * @returns {Promise<object>}
 */
async function registerAgent(ethAddress, alias, xnsHandle) {
  const response = await fetch(`${BASE_URL}/v1/agent/register`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      eth_address: ethAddress,
      alias,
      xns_handle: xnsHandle,
      description: "Registered via Node.js SDK",
    }),
  });
  if (!response.ok) throw new Error(`Registration failed: ${response.status}`);
  return response.json();
}

/**
 * Submit a telemetry report and receive updated AIS scores.
 * @param {object} params
 * @returns {Promise<object>}
 */
async function reportTransaction({
  agentId,
  dealId,
  dealAmount,
  latencyMs,
  accuracyScore,
  gpuHoursUsed = 0,
  hitlIntervention = false,
  performanceVariance = 0.05,
  verificationTier = 1,
}) {
  const response = await fetch(`${BASE_URL}/v1/transactions/report`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      agent_id: agentId,
      deal_id: dealId,
      deal_amount: dealAmount,
      latency_ms: latencyMs,
      accuracy_score: accuracyScore,
      gpu_hours_used: gpuHoursUsed,
      hitl_intervention: hitlIntervention,
      performance_variance: performanceVariance,
      verification_tier: verificationTier,
    }),
  });
  if (!response.ok) throw new Error(`Report failed: ${response.status}`);
  return response.json();
}

/**
 * Evaluate trust before executing a deal.
 * @param {string} initiatorEthAddress
 * @param {string} targetEthAddress
 * @returns {Promise<object>}
 */
async function performHandshake(initiatorEthAddress, targetEthAddress) {
  const response = await fetch(`${BASE_URL}/v1/agent/handshake`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      initiator_eth_address: initiatorEthAddress,
      target_eth_address: targetEthAddress,
    }),
  });
  if (!response.ok) throw new Error(`Handshake failed: ${response.status}`);
  return response.json();
}

/**
 * Resolve an XNS handle to a full agent profile.
 * @param {string} handle
 * @returns {Promise<object>}
 */
async function resolveXns(handle) {
  const response = await fetch(`${BASE_URL}/v1/identity/xns/${handle}`);
  if (!response.ok) throw new Error(`XNS resolution failed: ${response.status}`);
  return response.json();
}

/**
 * Get the W3C DID Document for an agent.
 * @param {string} ethAddress
 * @returns {Promise<object>}
 */
async function resolveDid(ethAddress) {
  const response = await fetch(`${BASE_URL}/v1/identity/did/${ethAddress}`);
  if (!response.ok) throw new Error(`DID resolution failed: ${response.status}`);
  return response.json();
}

// --- Example Usage ---

(async () => {
  try {
    // 1. Register an agent
    const agent = await registerAgent(
      "0xAbCd1234AbCd1234AbCd1234AbCd1234AbCd1234",
      "Nexus-Bot",
      "nexus"
    );
    console.log(`Registered: ${agent.did}`);
    console.log(`Agent ID:   ${agent.agent_id}`);

    // 2. Perform a handshake before dealing
    const handshake = await performHandshake(
      "0xAbCd1234AbCd1234AbCd1234AbCd1234AbCd1234",
      "0x1111222233334444555566667777888899990000"
    );
    console.log(`Trust Decision: ${handshake.trust_decision}`);
    console.log(`Handshake Hash: ${handshake.handshake_hash}`);

    if (handshake.trust_decision === "APPROVED") {
      // 3. Report the transaction result
      const score = await reportTransaction({
        agentId: agent.agent_id,
        dealId: "deal-node-001",
        dealAmount: 2500.0,
        latencyMs: 185,
        accuracyScore: 0.99,
        gpuHoursUsed: 8.5,
        hitlIntervention: true,
        performanceVariance: 0.01,
        verificationTier: 3,
      });
      console.log(`AIS Score:   ${score.ais_score}`);
      console.log(`Entropy:     ${score.entropy.toFixed(1)}`);
      console.log(`Grounding:   ${score.grounding.toFixed(1)}`);
      console.log(`Sacrifice:   ${score.sacrifice.toFixed(1)}`);
    }

    // 4. Resolve XNS
    const profile = await resolveXns("nexus");
    console.log(`XNS: ${profile.xns_handle} → ${profile.eth_address}`);
    console.log(`Trust Level: ${profile.trust_level}`);

    // 5. Fetch DID Document
    const didDoc = await resolveDid("0xAbCd1234AbCd1234AbCd1234AbCd1234AbCd1234");
    console.log(`DID: ${didDoc.id}`);
    console.log(`alsoKnownAs: ${didDoc.alsoKnownAs?.join(", ")}`);

  } catch (err) {
    console.error("Error:", err.message);
    process.exit(1);
  }
})();
```

---

*© 2026 Xibalba Protocol. All rights reserved. Documentation version 1.0.0 — generated 2026-05-31.*
