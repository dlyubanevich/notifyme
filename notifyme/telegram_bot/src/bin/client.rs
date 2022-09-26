use std::sync::Arc;

use domain::{models::UserId, requests::ClientRequest};
use dotenv::dotenv;
use rabbitmq_client::{Client, Publisher};
use telegram_bot::{
    client::{response_delegate, state::State, MessageHandler, Service},
    storage::StateStorage,
};

use teloxide::{
    dispatching::{update_listeners::webhooks, UpdateFilterExt},
    dptree,
    prelude::{AutoSend, Dispatcher, LoggingErrorHandler},
    requests::RequesterExt,
    types::{Message, Update},
    Bot,
};

use tokio::sync::Mutex;
use url::Url;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_APP_LOG", "info");
    pretty_env_logger::init_custom_env("RUST_APP_LOG");

    log::info!("Starting purchase bot...");

    dotenv().ok();

    let token = std::env::var("CLIENT_TOKEN").unwrap();

    let bot = Bot::new(token).auto_send();
    let addr = ([127, 0, 0, 1], 8080).into();
    let url = Url::parse("https://06df-95-27-196-93.eu.ngrok.io").unwrap();
    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
        .await
        .expect("Couldn't setup webhook");

    let service = Service::new(bot.clone());
    let queue_handler = Arc::new(Mutex::new(MessageHandler::new(service)));

    let response_queue = std::env::var("CLIENT_RESPONSE_QUEUE").unwrap();
    let mut client = Client::new("amqp://localhost:5672").await;
    client
        .with_consumer(&response_queue, response_delegate(queue_handler))
        .await;

    let message_handler = Update::filter_message().endpoint(message_handler);

    let handler = dptree::entry().branch(message_handler);

    let state_storage = StateStorage::<State>::new();
    let params = ConfigParams::new(client.get_publisher());

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![state_storage, params])
        .enable_ctrlc_handler()
        .build()
        .dispatch_with_listener(
            listener,
            LoggingErrorHandler::with_custom_text("main::An error from the update listener"),
        )
        .await;
}

#[derive(Clone)]
struct ConfigParams {
    publisher: Arc<Mutex<Publisher>>,
    exchange: String,
    request_queue: String,
}
impl ConfigParams {
    fn new(publisher: Arc<Mutex<Publisher>>) -> Self {
        let exchange = std::env::var("EXCHANGE").unwrap();
        let request_queue = std::env::var("CLIENT_REQUEST_QUEUE").unwrap();
        ConfigParams {
            publisher,
            exchange,
            request_queue,
        }
    }
}

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

async fn message_handler(
    bot: AutoSend<Bot>,
    msg: Message,
    storage: Arc<StateStorage<State>>,
    params: ConfigParams,
) -> HandlerResult {
    log::info!("Message from user [{}]", msg.chat.id.0);
    let state = match storage.get_state(&msg.chat.id).await {
        Some(state) => state,
        None => State::Start,
    };

    match state {
        State::Start => choose_customer(msg, storage, params).await?,
        State::Customer => choose_product(msg, storage, params).await?,
        State::Product { customer } => add_subscription(msg, storage, params, customer).await?,
        State::End => choose_customer(msg, storage, params).await?, //Костыль
    }

    Ok(())
}

async fn choose_customer(
    msg: Message,
    storage: Arc<StateStorage<State>>,
    params: ConfigParams,
) -> HandlerResult {
    log::info!("Choose customer for user [{}]", msg.chat.id.0);
    let message = ClientRequest::Customers {
        user_id: UserId::from(msg.chat.id.0),
        timestamp: msg.date.timestamp(),
    }
    .to_string();

    params
        .publisher
        .lock()
        .await
        .publish_message(&params.exchange, &params.request_queue, message)
        .await;
    storage.set_state(msg.chat.id, State::Customer).await;
    Ok(())
}

async fn choose_product(
    msg: Message,
    storage: Arc<StateStorage<State>>,
    params: ConfigParams,
) -> HandlerResult {
    log::info!("choose product for user [{}]", msg.chat.id.0);
    let customer = String::from(msg.text().unwrap());
    let message = ClientRequest::Products {
        user_id: UserId::from(msg.chat.id.0),
        customer: customer.clone(),
        timestamp: msg.date.timestamp(),
    }
    .to_string();
    params
        .publisher
        .lock()
        .await
        .publish_message(&params.exchange, &params.request_queue, message)
        .await;
    storage
        .set_state(msg.chat.id, State::Product { customer })
        .await;
    Ok(())
}

async fn add_subscription(
    msg: Message,
    storage: Arc<StateStorage<State>>,
    params: ConfigParams,
    customer: String,
) -> HandlerResult {
    log::info!("add subscription for user [{}]", msg.chat.id.0);
    let product = String::from(msg.text().unwrap());
    let message = ClientRequest::NewSubscription {
        user_id: UserId::from(msg.chat.id.0),
        customer,
        product,
        timestamp: msg.date.timestamp(),
    }
    .to_string();
    params
        .publisher
        .lock()
        .await
        .publish_message(&params.exchange, &params.request_queue, message)
        .await;
    storage.set_state(msg.chat.id, State::End).await;
    Ok(())
}
