use tonic::{Request, Response, Status};
use crate::proto::p2p::p2p_node_server::P2pNode;
use crate::proto::p2p::{
    HealthCheckRequest, HealthCheckResponse,
    SubmitTransactionRequest, SubmitTransactionResponse,
    SubmitBlockRequest, SubmitBlockResponse,
    SyncBlocksRequest, SyncBlocksResponse
};

#[derive(Debug, Default)]
pub struct NodeGrpcService;

#[tonic::async_trait]
impl P2pNode for NodeGrpcService {
    // 1. HealthCheck
    async fn health_check(
        &self,
        request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        println!("Got health_check request: {:?}", request);
        
        Ok(Response::new(HealthCheckResponse {
            healthy: true,
            version: "0.9.1".to_string(),
            // ????????? ??????????? ???? (???? ???????? 0)
            block_height: 0,
        }))
    }

    // 2. SubmitTransaction
    async fn submit_transaction(
        &self,
        _req: Request<SubmitTransactionRequest>,
    ) -> Result<Response<SubmitTransactionResponse>, Status> {
        Ok(Response::new(SubmitTransactionResponse {
            success: true,
            message: "Transaction accepted".to_string(),
            // ????????? ??????????? ???? (Mock hash)
            tx_hash: "mock_tx_hash_123".to_string(),
        }))
    }

    // 3. SubmitBlock
    async fn submit_block(
        &self,
        _req: Request<SubmitBlockRequest>,
    ) -> Result<Response<SubmitBlockResponse>, Status> {
        Ok(Response::new(SubmitBlockResponse {
            success: true,
            message: "Block accepted".to_string(),
            // ????????? ??????????? ???? (Mock hash)
            block_hash: "mock_block_hash_456".to_string(),
        }))
    }

    // 4. SyncBlocks
    async fn sync_blocks(
        &self,
        _req: Request<SyncBlocksRequest>,
    ) -> Result<Response<SyncBlocksResponse>, Status> {
        Ok(Response::new(SyncBlocksResponse {
            // ?????? ???? success/message, ??????? ??? ? Proto
            // ???????? ?????? ?????? ??????
            blocks: vec![],
            // ???????? ????
            has_more: false,
        }))
    }
}
