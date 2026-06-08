import urllib.request
import urllib.error
import json
import time
import subprocess
import os

URL = "http://127.0.0.1:3001/v1/transactions/verify"

def send_request(payload):
    req = urllib.request.Request(
        URL,
        data=json.dumps(payload).encode('utf-8'),
        headers={'Content-Type': 'application/json'}
    )
    try:
        with urllib.request.urlopen(req) as res:
            return res.status, json.loads(res.read().decode('utf-8'))
    except urllib.error.HTTPError as e:
        try:
            body = json.loads(e.read().decode('utf-8'))
        except Exception:
            body = e.reason
        return e.code, body
    except Exception as e:
        return 0, str(e)

def run_tests():
    print("=== Simulating API Edge Cases & Extremes ===\n")

    # Edge Case 1: Valid simulation payload (agent_id starts with 'agent_')
    print("1. Testing valid simulation payload...")
    valid_payload = {
        "agent_id": "agent_test_99",
        "zk_proof": "mock_proof_12345",
        "nonce": int(time.time() * 1000),
        "batch_size": 10,
        "payload_type": "telemetry"
    }
    code, res = send_request(valid_payload)
    print(f"   Response Code: {code}")
    print(f"   Response Body: {json.dumps(res, indent=2)}\n")

    # Edge Case 2: Replay attack (reuse the same nonce)
    print("2. Testing replay attack (submitting same nonce again)...")
    code, res = send_request(valid_payload)
    print(f"   Response Code: {code}")
    print(f"   Response Body: {json.dumps(res, indent=2)}\n")

    # Edge Case 3: Missing signature for a non-simulation agent
    print("3. Testing missing signature for non-simulation agent...")
    invalid_sig_payload = {
        "agent_id": "production_agent_01",
        "zk_proof": "mock_proof_12345",
        "nonce": int(time.time() * 1000) + 1,
        "batch_size": 5,
        "payload_type": "telemetry"
    }
    code, res = send_request(invalid_sig_payload)
    print(f"   Response Code: {code}")
    print(f"   Response Body: {json.dumps(res, indent=2)}\n")

    # Edge Case 4: Invalid/corrupt signature
    print("4. Testing invalid/corrupt signature format...")
    corrupt_sig_payload = {
        "agent_id": "production_agent_01",
        "zk_proof": "mock_proof_12345",
        "nonce": int(time.time() * 1000) + 2,
        "batch_size": 5,
        "payload_type": "telemetry",
        "signature": "invalid_hex_string"
    }
    code, res = send_request(corrupt_sig_payload)
    print(f"   Response Code: {code}")
    print(f"   Response Body: {json.dumps(res, indent=2)}\n")

    # Edge Case 5: Extremely large payload sizes (extreme inputs)
    print("5. Testing extreme payload size (massive ZK proof)...")
    extreme_payload = {
        "agent_id": "agent_test_extreme",
        "zk_proof": "A" * 100000, # 100KB ZK proof
        "nonce": int(time.time() * 1000) + 3,
        "batch_size": 99999,
        "payload_type": "telemetry"
    }
    code, res = send_request(extreme_payload)
    print(f"   Response Code: {code}")
    print(f"   Response Body: {json.dumps(res, indent=2)}\n")

if __name__ == "__main__":
    run_tests()
