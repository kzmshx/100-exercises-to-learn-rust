use crate::ticket::TicketStore;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type Db = Arc<RwLock<TicketStore>>;

pub fn create_db() -> Db {
    Arc::new(RwLock::new(TicketStore::default()))
}
