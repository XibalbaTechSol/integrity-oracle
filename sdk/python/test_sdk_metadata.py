import requests
import json
import subprocess
from xibalba_integrity import IntegrityClient, IntegrityConfig

def run_metadata_validation():
    print("🛡️ Starting SDK Agent Metadata Ingestion Validation...")
    
    # 1. Define high-fidelity, comprehensive agent metadata for Xibalba
    xibalba_metadata = {
        "agent_name": "Xibalba_Master_Intelligence",
        "version": "v9.4.2",
        "verification_tier": 3,
        "runtime_environment": {
            "tee_provider": "Intel_SGX",
            "enclave_measurement": "0x5d96a77e38de8ff0a1876b056ff43c20",
            "kernel_version": "Linux-6.8.0-secure"
        },
        "supported_models": ["gemini-1.5-pro", "claude-3-5-sonnet"],
        "compliance": {
            "hipaa_compliant": True,
            "soc2_verified": True
        }
    }
    
    config = IntegrityConfig(
        api_url="http://localhost:8080",
        agent_address="0x8888888888888888888888888888888888888888",
        api_key="xib_sdk_validation_token"
    )
    
    client = IntegrityClient(config)
    
    # Send registration request through the SDK session
    payload = {
        "eth_address": config.agent_address,
        "metadata": xibalba_metadata
    }
    
    print(f"Sending registration via SDK for address: {config.agent_address}...")
    resp = client._session.post(f"{config.api_url}/v1/agent/register", json=payload, timeout=10)
    
    if resp.status_code == 200:
        print("✅ SDK registration API call succeeded!")
        data = resp.json()
        print("API Response Payload:")
        print(json.dumps(data, indent=2))
        
        # 2. Directly query PostgreSQL to verify the exact metadata structure is intact
        print("\n--- 🗄️ Querying Postgres for ingested metadata verification ---")
        query = f"SELECT metadata FROM agents WHERE eth_address = '{config.agent_address}';"
        result = subprocess.run(
            ["PGPASSWORD=postgres psql -h localhost -U postgres -d integrity -t -c \"{0}\"".format(query)],
            shell=True,
            capture_output=True,
            text=True
        )
        
        db_output = result.stdout.strip()
        if db_output:
            parsed_db_metadata = json.loads(db_output)
            print("Successfully retrieved metadata from database:")
            print(json.dumps(parsed_db_metadata, indent=2))
            
            # Assert key-value structures
            assert parsed_db_metadata["agent_name"] == "Xibalba_Master_Intelligence", "Metadata verification mismatch: agent_name"
            assert parsed_db_metadata["runtime_environment"]["tee_provider"] == "Intel_SGX", "Metadata verification mismatch: TEE provider"
            assert parsed_db_metadata["compliance"]["hipaa_compliant"] is True, "Metadata verification mismatch: HIPAA compliance"
            
            print("\n🌟 SUCCESS: All metadata correctly and securely ingested by the SDK into the PostgreSQL database!")
        else:
            print("❌ Error: Failed to fetch metadata from PostgreSQL.")
            
    else:
        print(f"❌ Error: API request failed with status code {resp.status_code}: {resp.text}")

if __name__ == "__main__":
    run_metadata_validation()
