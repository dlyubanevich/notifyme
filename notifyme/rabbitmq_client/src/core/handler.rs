use std::{future::Future, pin::Pin};

pub trait IncomingMessageHandler: Send + Sync {
    fn handle_message(&mut self, message: String) -> Pin<Box<dyn Future<Output = ()> + Send>>;
}

impl<
        F: Future<Output = ()> + Send + 'static,
        MessageHandler: Fn(String) -> F + Send + Sync + 'static,
    > IncomingMessageHandler for MessageHandler
{
    fn handle_message(&mut self, message: String) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(self(message))
    }
}
