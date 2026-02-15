use tonic::{Request, Response, Status};
use chudo_core::{Block, BlockchainState, Transaction};
use crate::proto::blockchain_service_server::BlockchainService;
use crate::proto::{
    BlockRequest, BlockResponse, 
    TransactionResponse,
    BalanceRequest, BalanceResponse,
    SubmitTransactionRequest, SubmitTransactionResponse,
    MineBlockRequest, MineBlockResponse,
    ChainInfoRequest, ChainInfoResponse,
    SubscribeBlocksRequest,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_stream::wrappers::ReceiverStream;

pub struct GrpcService {
    pub state: Arc<RwLock<BlockchainState>>,
}

#[tonic::async_trait]
impl BlockchainService for GrpcService {
    // --- Получение баланса ---
    async fn get_balance(
        &self,
        request: Request<BalanceRequest>,
    ) -> Result<Response<BalanceResponse>, Status> {
        let req = request.into_inner();
        let state = self.state.read().await;
        
        let address_bytes = hex::decode(&req.address)
            .map_err(|_| Status::invalid_argument("Invalid address format"))?;

        // Пытаемся превратить Vec<u8> в [u8; 32]
        let address_array: [u8; 32] = address_bytes.try_into()
            .map_err(|_| Status::invalid_argument("Address must be 32 bytes"))?;

        let balance = state.get_balance(&address_array);

        Ok(Response::new(BalanceResponse {
            balance,
            nonce: 0, 
        }))
    }

    // --- Отправка транзакции ---
    async fn submit_transaction(
        &self,
        request: Request<SubmitTransactionRequest>,
    ) -> Result<Response<SubmitTransactionResponse>, Status> {
        let req = request.into_inner();
        
        let from = hex::decode(&req.from).map_err(|_| Status::invalid_argument("Invalid sender"))?;
        let to = hex::decode(&req.to).map_err(|_| Status::invalid_argument("Invalid recipient"))?;
        let signature = hex::decode(&req.signature).map_err(|_| Status::invalid_argument("Invalid signature"))?;

        let from_arr: [u8; 32] = from.try_into().map_err(|_| Status::invalid_argument("Invalid sender len"))?;
        let to_arr: [u8; 32] = to.try_into().map_err(|_| Status::invalid_argument("Invalid recipient len"))?;

        // Исправленный конструктор (5 аргументов)
        let mut tx = Transaction::new(
            from_arr,
            to_arr,
            req.amount,
            req.fee,
            req.nonce,
        );

        tx.signature = Some(signature);
        tx.calculate_hash(); 

        let tx_hash = hex::encode(&tx.hash);

        let mut state = self.state.write().await;
        state.add_transaction(tx)
             .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(SubmitTransactionResponse {
            hash: tx_hash,
            status: "Pending".to_string(),
        }))
    }

    // --- Майнинг ---
    async fn mine_block(
        &self,
        request: Request<MineBlockRequest>,
    ) -> Result<Response<MineBlockResponse>, Status> {
        let req = request.into_inner();
        let miner = hex::decode(&req.miner_address).map_err(|_| Status::invalid_argument("Invalid miner address"))?;
        let miner_arr: [u8; 32] = miner.try_into().map_err(|_| Status::invalid_argument("Invalid miner len"))?;
        
        let mut state = self.state.write().await;
        let block = state.mine_block(miner_arr); 

        Ok(Response::new(MineBlockResponse {
            block_hash: hex::encode(&block.hash),
            height: block.header.height, 
            transactions_count: block.transactions.len() as u64,
        }))
    }

    // --- Инфо о блоке ---
    async fn get_block(
        &self,
        request: Request<BlockRequest>,
    ) -> Result<Response<BlockResponse>, Status> {
        let req = request.into_inner();
        let hash = hex::decode(&req.hash).map_err(|_| Status::invalid_argument("Invalid hash"))?;
        let hash_arr: [u8; 32] = hash.try_into().map_err(|_| Status::invalid_argument("Invalid hash len"))?;
        
        let state = self.state.read().await;
        
        if let Some(block) = state.get_block(&hash_arr) {
            Ok(Response::new(map_block_to_proto(block)))
        } else {
            Err(Status::not_found("Block not found"))
        }
    }

    // --- Инфо о сети ---
    async fn get_chain_info(
        &self,
        _request: Request<ChainInfoRequest>,
    ) -> Result<Response<ChainInfoResponse>, Status> {
        let state = self.state.read().await;
        Ok(Response::new(ChainInfoResponse {
            height: state.current_height(), 
            total_supply: 100_000_000, // Заглушка, можно брать из state если реализовано
            peer_count: 0,
            is_syncing: false,
        }))
    }

    type SubscribeBlocksStream = ReceiverStream<Result<BlockResponse, Status>>;
    
    async fn subscribe_blocks(
        &self,
        _request: Request<SubscribeBlocksRequest>,
    ) -> Result<Response<Self::SubscribeBlocksStream>, Status> {
        let (_tx, rx) = tokio::sync::mpsc::channel(1);
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

fn map_block_to_proto(block: &Block) -> BlockResponse {
    BlockResponse {
        hash: hex::encode(&block.hash),
        prev_hash: hex::encode(&block.header.prev_hash),
        height: block.header.height,
        timestamp: block.header.timestamp as u64,
        nonce: block.header.nonce,
        miner: hex::encode(&block.header.miner),
        transactions: block.transactions.iter().map(map_tx_to_proto).collect(),
    }
}

fn map_tx_to_proto(tx: &Transaction) -> TransactionResponse {
    TransactionResponse {
        hash: hex::encode(&tx.hash),
        from: hex::encode(&tx.sender),
        to: hex::encode(&tx.recipient),
        amount: tx.amount,
        fee: tx.fee,
        nonce: tx.nonce,
        signature: hex::encode(tx.signature.as_ref().unwrap_or(&vec![])),
        timestamp: tx.timestamp as u64,
    }
}
