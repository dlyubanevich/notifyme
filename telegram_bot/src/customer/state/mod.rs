#[derive(Debug, Clone)]
pub enum State {
    Start,
    Authorization,
    Command,
    AddNotification { customer: String },
    SendNotification { customer: String, product: String },
}
