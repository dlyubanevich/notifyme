#[derive(Clone)]
pub enum State {
    Start,
    Authorization,
    Command,
    Subscriptions,
    Notification,
}
