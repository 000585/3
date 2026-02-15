pub mod encryption;
pub mod identity;
pub mod network;
pub mod protocol;
pub mod storage;
pub mod incentive;

pub use lib_impl::*;

mod lib_impl {
    use anyhow::Result;
    use std::sync::Arc;
    use tokio::sync::{RwLock, mpsc};
    use tracing::{info, debug, warn, error};
    
    pub struct MessengerNode;
    
    impl MessengerNode {
        pub async fn new() -> Result<Self> {
            info!("CHUDO Messenger initializing...");
            Ok(Self)
        }
    }
}
