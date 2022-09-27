use std::{collections::HashMap, sync::Arc};

use teloxide::types::ChatId;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct StateStorage<V> {
    map: Mutex<HashMap<ChatId, V>>,
}

impl<V> StateStorage<V>
where
    V: Clone,
{
    #[must_use]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            map: Mutex::new(HashMap::new()),
        })
    }

    pub async fn get_state(&self, chat_id: &ChatId) -> Option<V> {
        self.map.lock().await.get(chat_id).map(ToOwned::to_owned)
    }
    pub async fn set_state(&self, chat_id: ChatId, state: V) {
        self.map.lock().await.insert(chat_id, state);
    }
}
