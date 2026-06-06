use alloy_primitives::{Address, FixedBytes};
use alloy_provider::{Provider, ProviderBuilder, RootProvider};
use alloy_signer_local::PrivateKeySigner;
use alloy_network::EthereumWallet;
use std::sync::Arc;
use alloy_transport_http::Http;
use reqwest::Client;

// Define StateAnchor interface
alloy_sol_types::sol! {
    #[sol(rpc)]
    contract StateAnchor {
        function updateStateRoot(bytes32 newRoot) external;
        function getLatestRoot() external view returns (bytes32);
    }
}

pub struct RollupDaemon {
    contract_address: Address,
    provider: Arc<dyn Provider>,
}

impl RollupDaemon {
    pub async fn new(rpc_url: &str, contract_addr: Address, private_key: &str) -> Self {
        let signer: PrivateKeySigner = private_key.parse().unwrap();
        let wallet = EthereumWallet::from(signer);
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .on_http(rpc_url.parse().unwrap());
            
        Self {
            contract_address: contract_addr,
            provider: Arc::new(provider),
        }
    }

    pub async fn commit_root(&self, root: [u8; 32]) -> Result<String, String> {
        let contract = StateAnchor::new(self.contract_address, Arc::clone(&self.provider));
        let fixed_root = FixedBytes::from_slice(&root);
        
        let tx = contract.updateStateRoot(fixed_root)
            .send()
            .await
            .map_err(|e| format!("Transaction send failed: {:?}", e))?;
            
        let receipt = tx.get_receipt()
            .await
            .map_err(|e| format!("Failed to retrieve receipt: {:?}", e))?;
            
        Ok(receipt.transaction_hash.to_string())
    }
}
