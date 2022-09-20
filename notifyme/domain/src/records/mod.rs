use serde::{Deserialize, Serialize};

use crate::models::{CustomerEventRecord, UserEventRecord};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Record {
    UserEvent(UserEventRecord),
    CustomerEvent(CustomerEventRecord),
}

impl ToString for Record {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
