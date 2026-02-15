use tonic::transport::Channel;
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::Result;

// ???????? ??????????????? ???
pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/chudo.rs"));
}

use proto::{
    p2p_node_client::P2pNodeClient,
    SubmitTransactionRequest, SubmitBlockRequest, 
    SyncBlocksRequest, HealthCheckRequest,
    SubmitTransactionResponse, SubmitBlockResponse,
    SyncBlocksResponse, HealthCheckResponse,
};

pub struct ChudoGrpcClient {
    client: P2pNodeClient<Channel>,
}

impl ChudoGrpcClient {
    pub async fn connect(addr: &str) -> Result<Self> {
        let client = P2pNodeClient::connect(addr.to_string()).await?;
        Ok(Self { client })
    }

    pub async fn health_check(&mut self) -> Result<HealthCheckResponse> {
        let request = tonic::Request::new(HealthCheckRequest {});
        let response: tonic::Response<HealthCheckResponse> = self.client.health_check(request).await?;
        Ok(response.into_inner())
    }

    pub async fn submit_transaction_raw(&mut self, tx_bytes: Vec<u8>, tx_hash: String) -> Result<SubmitTransactionResponse> {
        let request = tonic::Request::new(SubmitTransactionRequest {
            transaction_data: tx_bytes,
            tx_hash,
        });
        let response: tonic::Response<SubmitTransactionResponse> = self.client.submit_transaction(request).await?;
        Ok(response.into_inner())
    }

    pub async fn submit_block_raw(&mut self, block_bytes: Vec<u8>, block_hash: String, height: u64) -> Result<SubmitBlockResponse> {
        let request = tonic::Request::new(SubmitBlockRequest {
            block_data: block_bytes,
            block_hash,
            height,
        });
        let response: tonic::Response<SubmitBlockResponse> = self.client.submit_block(request).await?;
        Ok(response.into_inner())
    }

    pub async fn sync_blocks(&mut self, from_height: u64, to_height: u64) -> Result<SyncBlocksResponse> {
        let request = tonic::Request::new(SyncBlocksRequest {
            from_height,
            to_height,
        });
        let response: tonic::Response<SyncBlocksResponse> = self.client.sync_blocks(request).await?;
        Ok(response.into_inner())
    }
}

fn create_test_transaction_bytes() -> (Vec<u8>, String) {
    let test_payload = format!("test_tx_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis());
    let bytes = test_payload.as_bytes().to_vec();
    let hash = format!("{:x}", md5::compute(&bytes));
    (bytes, hash)
}

fn create_test_block_bytes(height: u64) -> (Vec<u8>, String) {
    let test_payload = format!("test_block_{}_{}", height, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis());
    let bytes = test_payload.as_bytes().to_vec();
    let hash = format!("{:x}", md5::compute(&bytes));
    (bytes, hash)
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("?? CHUDO gRPC Test Client v0.8.0");
    println!("=================================");
    
    let addr = std::env::var("CHUDO_NODE_ADDR")
        .unwrap_or_else(|_| "http://[::1]:50051".to_string());
    
    println!("?? ??????????? ?: {}", addr);
    let mut client = ChudoGrpcClient::connect(&addr).await?;
    
    println!("\n?? ???? 1: Health Check");
    match client.health_check().await {
        Ok(health) => {
            println!("   ? ???? ???????: {}", health.healthy);
            println!("   ?? ??????: {}", health.version);
            println!("   ??  ??????: {}", health.block_height);
        }
        Err(e) => println!("   ? ??????: {}", e),
    }
    
    println!("\n?? ???? 2: Sync Blocks (0-10)");
    match client.sync_blocks(0, 10).await {
        Ok(sync) => {
            println!("   ? ???????? ??????: {}", sync.blocks.len());
        }
        Err(e) => println!("   ? ??????: {}", e),
    }
    
    println!("\n?? ???? 3: Submit Transaction");
    let (tx_bytes, tx_hash) = create_test_transaction_bytes();
    match client.submit_transaction_raw(tx_bytes, tx_hash).await {
        Ok(response) => {
            println!("   ? ??????: {}", if response.success { "???????" } else { "?????????" });
            if !response.message.is_empty() {
                println!("   ?? {}", response.message);
            }
        }
        Err(e) => println!("   ? ??????: {}", e),
    }
    
    println!("\n?? ???? 4: Submit Block");
    let (block_bytes, block_hash) = create_test_block_bytes(1);
    match client.submit_block_raw(block_bytes, block_hash, 1).await {
        Ok(response) => {
            println!("   ? ??????: {}", if response.success { "??????" } else { "????????" });
            if !response.message.is_empty() {
                println!("   ?? {}", response.message);
            }
        }
        Err(e) => println!("   ? ??????: {}", e),
    }
    
    println!("\n? ??? ????? ?????????!");
    Ok(())
}
