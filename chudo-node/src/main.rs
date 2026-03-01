use tonic::{transport::Server, Request, Response, Status};
use std::sync::{Arc, Mutex};

pub mod chudo {
    tonic::include_proto!("chudo");
}

#[derive(Debug, Default)]
pub struct MyNode {
    block_height: Arc<Mutex<u64>>,
}

#[tonic::async_trait]
impl chudo::node_service_server::NodeService for MyNode {
    async fn get_status(&self, _request: Request<chudo::Empty>) -> Result<Response<chudo::NodeStatus>, Status> {
        let height = *self.block_height.lock().unwrap();
        Ok(Response::new(chudo::NodeStatus {
            running: true,
            block_height: height,
            peer_count: 21,
            last_block_hash: "0000abc123...".to_string(),
        }))
    }
    async fn send_command(&self, _r: Request<chudo::CommandRequest>) -> Result<Response<chudo::CommandResponse>, Status> {
        Ok(Response::new(chudo::CommandResponse { success: true, message: "OK".into() }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50054".parse()?;
    let node = MyNode::default();
    let height_clone = node.block_height.clone();
    
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            let mut h = height_clone.lock().unwrap();
            *h += 1;
        }
    });

    println!("?? CHUDO API listening on 127.0.0.1:50054");
    Server::builder().add_service(chudo::node_service_server::NodeServiceServer::new(node)).serve(addr).await?;
    Ok(())
}
