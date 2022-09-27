use std::{net::SocketAddr, sync::Arc};

use domain::{models::UserId, requests::ClientRequest};
use dotenv::dotenv;
use rabbitmq_client::{Publisher, RabbitMqManager};
use telegram_bot::{
    client::{state::State, ClientService, MessageHandler},
    storage::StateStorage,
    Config,
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
    dotenv().ok();
    pretty_env_logger::init();

    log::info!("Starting purchase bot...");

    let config = envy::from_env::<Config>().unwrap();

    let bot = Bot::new(&config.telegram_client_token).auto_send();
    let address: SocketAddr = config.telegram_client_address.parse().unwrap();
    let url = Url::parse(&config.telegram_client_url).unwrap();
    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(address, url))
        .await
        .expect("Couldn't setup webhook");

    let service = Arc::new(Mutex::new(ClientService::new(bot.clone())));

    let mut manager = RabbitMqManager::builder()
        .build(&config.amqp_address)
        .await
        .unwrap();
    manager
        .add_consumer(
            &config.client_response_queue,
            MessageHandler::client_response(service.clone()),
        )
        .await
        .unwrap();

    let publisher = Arc::new(Mutex::new(manager.get_publisher().await.unwrap()));
    let params = ConfigParams::new(config, publisher);
    let state_storage = StateStorage::<State>::new();

    let telegram_message_handler = Update::filter_message().endpoint(message_handler);
    let handler = dptree::entry().branch(telegram_message_handler);

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
    fn new(config: Config, publisher: Arc<Mutex<Publisher>>) -> Self {
        let exchange = config.exchange;
        let request_queue = config.client_request_queue;
        ConfigParams {
            publisher,
            exchange,
            request_queue,
        }
    }
}

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

async fn message_handler(
    _bot: AutoSend<Bot>,
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
        .await
        .unwrap();
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
        .await
        .unwrap();
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
        .await
        .unwrap();
    storage.set_state(msg.chat.id, State::End).await;
    Ok(())
}
