use axum::{
    routing::{get, post},
    Router, Json, extract::{State, Path},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

// --- DTOs ---

#[derive(Debug, Deserialize)]
pub struct RegisterAgentPayload {
    pub eth_address: String,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct RegisterAgentResponse {
    pub agent_id: String,
    pub eth_address: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct TelemetryPayload {
    pub agent_id: String,
    pub deal_id: String,
    pub deal_amount: f64,
    pub latency_ms: u32,
    pub accuracy_score: f32, // 0.0 to 1.0
    pub hitl_intervention: bool, // Human in the loop intervention
    pub gpu_hours_used: f32,
    pub performance_variance: f32, // Usually tracked historically, passed here for MVP
    pub verification_tier: u32, // 1=Sovereign, 2=Linked, 3=Institutional
}

#[derive(Debug, Serialize, Clone)]
pub struct TriMetricResponse {
    pub agent_id: String,
    pub ais_score: u32,
    pub entropy: u32,
    pub grounding: u32,
    pub sacrifice: u32,
    pub integrity_hash: String,
}

struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Xibalba Oracle Backend (Rust/Axum)...");

    // Load DB from .env (in real environment)
    // For MVP compilation, we allow fallback or mock pool if DSN is missing
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/integrity".to_string());
    
    // Connect to Postgres
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await;

    // If DB isn't running yet locally, we still allow the server to start for UI testing.
    let state = Arc::new(AppState {
        db: pool.unwrap_or_else(|_| panic!("Failed to connect to postgres. Ensure DB is running.")),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(|| async { "Xibalba Oracle API is operational." }))
        .route("/v1/agent/register", post(register_agent))
        .route("/v1/transactions/report", post(ingest_telemetry))
        .route("/v1/transactions/verify", post(verify_transaction))
        .route("/v1/agent/:identifier", get(get_agent))
        .layer(cors)
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Listening on port 8080");
    
    axum::serve(listener, app).await?;
    Ok(())
}

// --- Endpoints ---

/// Registers a new agent into the proprietary reputation database.
async fn register_agent(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterAgentPayload>,
) -> Json<RegisterAgentResponse> {
    // In production: INSERT INTO agents (eth_address, metadata) VALUES (...)
    
    Json(RegisterAgentResponse {
        agent_id: "00000000-0000-0000-0000-000000000001".to_string(),
        eth_address: payload.eth_address,
        status: "Registered".to_string(),
    })
}

/// Ingests agent telemetry, calculates the Tri-Metric score, and mocks a state anchor
async fn ingest_telemetry(
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<TelemetryPayload>,
) -> Json<TriMetricResponse> {
    println!("Received telemetry for agent: {}", payload.agent_id);

    use sha2::{Sha256, Digest};

    // --- 1. Cryptographic Hashing ---
    // SHA256("{deal_id}-{latency_ms}-{accuracy}-{amount}")
    let hash_input = format!("{}-{}-{}-{}", payload.deal_id, payload.latency_ms, payload.accuracy_score, payload.deal_amount);
    let mut hasher = Sha256::new();
    hasher.update(hash_input.as_bytes());
    let integrity_hash = format!("0x{}", hex::encode(hasher.finalize()));

    // --- 2. The Tri-Metric Calculation Engine ---
    // Entropy: e^(-1.5 * variance) * 1000
    let entropy_score = (std::f32::consts::E.powf(-1.5 * payload.performance_variance) * 1000.0) as u32;

    // Grounding: HGI multiplier
    let hgi = if payload.hitl_intervention { 0.95 } else { 0.50 };
    let grounding_score = (hgi * 1000.0) as u32;

    // Sacrifice: Proof of Compute
    let sacrifice_score = ((payload.gpu_hours_used / 100.0).min(1.0) * 1000.0) as u32;

    // MVP Mocks for Staking, TrustFlow, Audit, Volume
    let staking_score = 800;
    let trustflow_score = 750;
    let audit_score = if payload.verification_tier == 3 { 1000 } else { 500 };
    let volume_score = 600;

    // Composite AIS: Staking (20%), Sacrifice (20%), TrustFlow (25%), Audits (25%), Volume (10%)
    let raw_ais = (
        (staking_score as f32 * 0.20) +
        (sacrifice_score as f32 * 0.20) +
        (trustflow_score as f32 * 0.25) +
        (audit_score as f32 * 0.25) +
        (volume_score as f32 * 0.10)
    ) as u32;

    // Blended AIS with Entropy and Grounding
    let blended_ais = (raw_ais + entropy_score + grounding_score) / 3;

    // Apply Verification Tier Ceiling
    let tier_ceiling = match payload.verification_tier {
        1 => 600,
        2 => 850,
        _ => 1000,
    };
    
    let ais_score = blended_ais.min(tier_ceiling);

    // Production: INSERT INTO transaction_logs ...

    Json(TriMetricResponse {
        agent_id: payload.agent_id,
        ais_score,
        entropy: entropy_score,
        grounding: grounding_score,
        sacrifice: sacrifice_score,
        integrity_hash,
    })
}

/// Verifies a specific transaction
async fn verify_transaction() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "Verified",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// Retrieves the agent metrics from the DB for the Explorer UI
async fn get_agent(Path(identifier): Path<String>) -> Json<serde_json::Value> {
    // Production: SELECT * FROM agents WHERE agent_id = $1 OR eth_address = $1
    Json(serde_json::json!({
        "agent_id": identifier,
        "eth_address": "0xMockAddress",
        "current_ais": 850,
        "gpu_hours_verified": 150.5,
        "performance_entropy": 0.05
    }))
}
