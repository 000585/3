use tonic::{Request, Response, Status};
use tokio_stream::wrappers::ReceiverStream;
use std::sync::Arc;

use crate::proto::messenger::messenger_server::Messenger;
use crate::proto::messenger::{
    SendMessageRequest, SendMessageResponse,
    SubscribeRequest, Message as ProtoMessage,
    HistoryRequest, HistoryResponse,
    IdentityRequest, IdentityResponse,
    ClaimRequest, ClaimResponse
};

use chudo_messenger::MessengerNode;
use chudo_messenger::MessengerEvent;

pub struct MessengerGrpcService {
    core: Arc<MessengerNode>,
}

impl MessengerGrpcService {
    pub fn new(core: Arc<MessengerNode>) -> Self {
        Self { core }
    }
}

#[tonic::async_trait]
impl Messenger for MessengerGrpcService {
    type SubscribeMessagesStream = ReceiverStream<Result<ProtoMessage, Status>>;

    async fn send_message(&self, request: Request<SendMessageRequest>) -> Result<Response<SendMessageResponse>, Status> {
        let req = request.into_inner();
        match self.core.send_message(&req.to, &req.content).await {
            Ok(id) => Ok(Response::new(SendMessageResponse {
                message_id: id,
                success: true,
                error: String::new(),
            })),
            Err(e) => Ok(Response::new(SendMessageResponse {
                message_id: String::new(),
                success: false,
                error: e.to_string(),
            })),
        }
    }

    async fn get_identity(&self, _req: Request<IdentityRequest>) -> Result<Response<IdentityResponse>, Status> {
        match self.core.get_identity().await {
            Ok(data) => {
                Ok(Response::new(IdentityResponse {
                    public_key: hex::encode(&data.public_key),
                    peer_id: data.peer_id,
                    nickname: data.nickname.unwrap_or_else(|| "Anon".to_string()),
                    encryption_public_key: hex::encode(&data.encryption_public_key),
                }))
            },
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_history(&self, request: Request<HistoryRequest>) -> Result<Response<HistoryResponse>, Status> {
        let req = request.into_inner();
        match self.core.get_history(&req.with_user, req.limit as usize).await {
            Ok(messages) => {
                let proto_msgs = messages.into_iter().map(|m| ProtoMessage {
                    id: m.id,
                    from: m.from,
                    to: m.to,
                    content: m.content,
                    timestamp: m.timestamp.timestamp(),
                    signature: hex::encode(m.signature),
                    channel: String::new(),
                    is_encrypted: m.is_encrypted,
                    nonce: m.nonce.map(|n| hex::encode(n)).unwrap_or_default(),
                }).collect();

                Ok(Response::new(HistoryResponse { messages: proto_msgs }))
            },
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn subscribe_messages(&self, _req: Request<SubscribeRequest>) -> Result<Response<Self::SubscribeMessagesStream>, Status> {
        let mut lib_rx = self.core.subscribe_messages().await;
        let (tx, rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            while let Some(event) = lib_rx.recv().await {
                if let MessengerEvent::MessageReceived(msg) = event {
                    let proto_msg = ProtoMessage {
                        id: msg.id,
                        from: msg.from,
                        to: msg.to,
                        content: msg.content,
                        timestamp: msg.timestamp.timestamp(),
                        signature: hex::encode(msg.signature),
                        channel: String::new(),
                        is_encrypted: msg.is_encrypted,
                        nonce: msg.nonce.map(|n| hex::encode(n)).unwrap_or_default(),
                    };
                    if tx.send(Ok(proto_msg)).await.is_err() {
                        break;
                    }
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn claim_rewards(&self, request: Request<ClaimRequest>) -> Result<Response<ClaimResponse>, Status> {
        let req = request.into_inner();
        match self.core.claim_rewards(&req.blockchain_address).await {
            Ok(reward) => Ok(Response::new(ClaimResponse {
                claim_id: hex::encode(reward.proof),
                amount: reward.amount as u64,
                success: true,
            })),
            Err(_) => Ok(Response::new(ClaimResponse {
                claim_id: String::new(),
                amount: 0,
                success: false,
            })),
        }
    }
}
