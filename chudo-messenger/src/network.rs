use std::sync::Arc;

pub struct P2PNetwork;
impl P2PNetwork {
    pub async fn new(_: Arc<tokio::sync::RwLock<crate::identity::Identity>>) -> anyhow::Result<Self> {
        Ok(Self)
    }
    pub async fn start(&self) -> anyhow::Result<()> { Ok(()) }
    pub async fn send_message(&self, _to: &str, _msg: &crate::protocol::Message) -> anyhow::Result<()> { Ok(()) }
}
