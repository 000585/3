pub struct MessageStorage;
impl MessageStorage {
    pub async fn new(_path: &str) -> anyhow::Result<Self> { Ok(Self) }
    pub async fn store_message(&self, _msg: &crate::protocol::Message) -> anyhow::Result<()> { Ok(()) }
    pub async fn get_conversation(&self, _me: &str, _other: &str, _limit: usize) -> anyhow::Result<Vec<crate::protocol::Message>> { Ok(vec![]) }
    pub async fn get_all_messages(&self, _limit: usize) -> anyhow::Result<Vec<crate::protocol::Message>> { Ok(vec![]) }
}
