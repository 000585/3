pub mod encryption;
pub mod identity;
pub mod network;
pub mod protocol;
pub mod storage;
pub mod incentive;

pub use lib_impl::*;

mod lib_impl {
    use anyhow::Result;
    use tokio::sync::mpsc;
    use tracing::info;

    #[derive(Clone)]
    pub struct IdentityData {
        pub peer_id: String,
        pub blockchain_address: String,
        pub public_key: Vec<u8>,
        pub encryption_public_key: Vec<u8>,
        pub nickname: Option<String>,
    }

    #[derive(Clone)]
    pub struct Message {
        pub id: String,
        pub from: String,
        pub to: String,
        pub content: String,
        pub timestamp: chrono::DateTime<chrono::Utc>,
        pub signature: Vec<u8>,
        pub is_encrypted: bool,
        pub nonce: Option<Vec<u8>>,
    }

    #[derive(Clone)]
    pub struct Reward {
        pub proof: Vec<u8>,
        pub amount: i64,
    }

    #[derive(Clone)]
    pub enum MessengerEvent {
        MessageReceived(Message),
        Connected(String),
        Disconnected(String),
    }

    pub struct MessengerNode;

    impl MessengerNode {
        pub async fn new() -> Result<Self> {
            info!("CHUDO Messenger initializing...");
            Ok(Self)
        }

        pub async fn new_with_db_path(_path: &str) -> Result<Self> {
            info!("CHUDO Messenger initializing with DB at {}...", _path);
            Ok(Self)
        }

        pub async fn start(&self) -> Result<()> {
            info!("CHUDO Messenger started");
            Ok(())
        }

        pub async fn send_message(&self, to: &str, content: &str) -> Result<String> {
            info!("Sending message to {}: {}", to, content);
            Ok(uuid::Uuid::new_v4().to_string())
        }

        pub async fn get_identity(&self) -> Result<IdentityData> {
            Ok(IdentityData {
                peer_id: "peer_123".to_string(),
                blockchain_address: "0x123".to_string(),
                public_key: vec![1, 2, 3],
                encryption_public_key: vec![4, 5, 6],
                nickname: Some("User".to_string()),
            })
        }

        pub async fn get_history(&self, _with_user: &str, _limit: usize) -> Result<Vec<Message>> {
            Ok(vec![])
        }

        pub async fn subscribe_messages(&self) -> mpsc::Receiver<MessengerEvent> {
            let (_tx, rx) = mpsc::channel(100);
            rx
        }

        pub async fn claim_rewards(&self, _address: &str) -> Result<Reward> {
            Ok(Reward {
                proof: vec![1, 2, 3],
                amount: 100,
            })
        }
    }
}
