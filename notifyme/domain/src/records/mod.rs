use serde::{Serialize, Deserialize};

use crate::models::{UserEventRecord, CustomerEventRecord};

#[derive(Serialize, Deserialize)]
pub enum Record {
    UserEvent(UserEventRecord),
    CustomerEvent(CustomerEventRecord),
}