use tonic::transport::Server;
use std::sync::Arc;
use chudo_messenger::MessengerNode; // ?????????? ??????????

use chudo_node::grpc_server::NodeGrpcService;
use chudo_node::messenger_server::MessengerGrpcService;
use chudo_node::proto::p2p::p2p_node_server::P2pNodeServer;
use chudo_node::proto::messenger::messenger_server::MessengerServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ???????? ???????????
    tracing_subscriber::fmt::init();

    let p2p_addr = "[::1]:50053".parse()?;
    let msg_addr = "[::1]:50054".parse()?;
    
    // --- 1. ????????????? Messenger Node (Lib) ---
    println!("Initializing CHUDO Messenger Core...");
    // ?????????? ?? ? ????? chudo_messenger_db
    let messenger_node = Arc::new(MessengerNode::new_with_db_path("chudo_messenger_db").await?);
    
    // ????????? ??????? ???? ??????????? (P2P/Mock)
    messenger_node.start().await?;
    
    // --- 2. ?????? P2P ??????? ????????? ---
    let node_service = NodeGrpcService::default();
    println!("Blockchain P2P Node listening on {}", p2p_addr);
    
    tokio::spawn(async move {
        Server::builder()
            .add_service(P2pNodeServer::new(node_service))
            .serve(p2p_addr)
            .await
            .unwrap();
    });

    // --- 3. ?????? Messenger gRPC ??????? ---
    // ???????? ???????? messenger_node ? gRPC ??????
    let messenger_service = MessengerGrpcService::new(messenger_node.clone());
    println!("Messenger gRPC API listening on {}", msg_addr);

    Server::builder()
        .add_service(MessengerServer::new(messenger_service))
        .serve(msg_addr)
        .await?;

    Ok(())
}
