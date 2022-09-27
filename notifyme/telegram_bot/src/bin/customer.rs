use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use domain::{
    models::{Customer, UserId},
    requests::CustomerRequest,
};
use dotenv::dotenv;
use rabbitmq_client::{Publisher, RabbitMqManager};
use serde::{Deserialize, Serialize};
use telegram_bot::{
    customer::{state::State, CustomerService, MessageHandler},
    storage::StateStorage,
    Config, HandlerResult,
};
use teloxide::{
    dispatching::{update_listeners::webhooks, UpdateFilterExt},
    dptree,
    payloads::SendMessageSetters,
    prelude::{AutoSend, Dispatcher, LoggingErrorHandler},
    requests::{Requester, RequesterExt},
    types::{
        CallbackQuery, ChatId, InlineKeyboardButton, InlineKeyboardMarkup, KeyboardRemove, Message,
        Update,
    },
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

    let bot = Bot::new(&config.telegram_customer_token).auto_send();
    let address: SocketAddr = config.telegram_customer_address.parse().unwrap();
    let url = Url::parse(&config.telegram_customer_url).unwrap();
    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(address, url))
        .await
        .expect("Couldn't setup webhook");

    let state_storage = StateStorage::<State>::new();
    let authorized_customers = Arc::new(Mutex::new(HashMap::<ChatId, Customer>::new()));
    let service = Arc::new(Mutex::new(CustomerService::new(
        bot.clone(),
        state_storage.clone(),
        authorized_customers.clone(),
    )));

    let mut manager = RabbitMqManager::builder()
        .build(&config.amqp_address)
        .await
        .unwrap();
    manager
        .add_consumer(
            &config.customer_response_queue,
            MessageHandler::customer_response(service.clone()),
        )
        .await
        .unwrap();

    let publisher = Arc::new(Mutex::new(manager.get_publisher().await.unwrap()));
    let params = ConfigParams::new(config, authorized_customers, publisher);

    let message_handler = Update::filter_message().endpoint(message_handler);
    let callback_query_handler = Update::filter_callback_query().endpoint(callback_handler);

    let handler = dptree::entry()
        .branch(message_handler)
        .branch(callback_query_handler);

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
    authorized_customers: Arc<Mutex<HashMap<ChatId, Customer>>>,
    publisher: Arc<Mutex<Publisher>>,
    exchange: String,
    request_queue: String,
}
impl ConfigParams {
    fn new(
        config: Config,
        authorized_customers: Arc<Mutex<HashMap<ChatId, Customer>>>,
        publisher: Arc<Mutex<Publisher>>,
    ) -> Self {
        let exchange = config.exchange;
        let request_queue = config.customer_request_queue;
        ConfigParams {
            authorized_customers,
            publisher,
            exchange,
            request_queue,
        }
    }
}

async fn message_handler(
    bot: AutoSend<Bot>,
    msg: Message,
    storage: Arc<StateStorage<State>>,
    params: ConfigParams,
) -> HandlerResult {
    let state = match storage.get_state(&msg.chat.id).await {
        Some(state) => state,
        None => State::Start,
    };
    log::info!("Message from user [{}], state: {:?}", msg.chat.id, state);
    match state {
        State::Start => start(bot, msg, storage, params).await?,
        State::Authorization => authorization(msg, params).await?,
        State::Command => choose_command(bot, msg).await?,
        State::AddNotification { customer } => {
            add_notification(bot, msg, storage, customer).await?
        }
        State::SendNotification { customer, product } => {
            send_notification(msg, params, customer, product).await?
        }
    }

    Ok(())
}

async fn start(
    bot: AutoSend<Bot>,
    msg: Message,
    storage: Arc<StateStorage<State>>,
    params: ConfigParams,
) -> HandlerResult {
    log::info!("Start for user [{}]", msg.chat.id.0);
    if params
        .authorized_customers
        .lock()
        .await
        .get(&msg.chat.id)
        .is_some()
    {
        storage.set_state(msg.chat.id, State::Command).await;
        choose_command(bot, msg).await?;
    } else {
        bot.send_message(msg.chat.id, "Введите ключ для авторизации:")
            .await?;
        storage.set_state(msg.chat.id, State::Authorization).await;
    }
    Ok(())
}

async fn authorization(msg: Message, params: ConfigParams) -> HandlerResult {
    log::info!("Authorization for user [{}]", msg.chat.id.0);
    let key = msg.text().unwrap().to_owned();
    let message = CustomerRequest::Authorization {
        user_id: UserId::from(msg.chat.id.0),
        key,
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
    Ok(())
}
async fn choose_command(bot: AutoSend<Bot>, msg: Message) -> HandlerResult {
    log::info!("Choose command for user [{}]", msg.chat.id.0);
    let keyboard: Vec<Vec<InlineKeyboardButton>> = vec![vec![InlineKeyboardButton::callback(
        "Создать уведомление".to_owned(),
        Command::AddNotification,
    )]];
    bot.send_message(msg.chat.id, "Что хотите сделать?")
        .reply_markup(InlineKeyboardMarkup::new(keyboard))
        .await?;

    Ok(())
}

async fn add_notification(
    bot: AutoSend<Bot>,
    msg: Message,
    storage: Arc<StateStorage<State>>,
    customer: String,
) -> HandlerResult {
    log::info!("add notification for user [{}]", msg.chat.id.0);
    let product = msg.text().unwrap().to_string();
    bot.send_message(
        msg.chat.id,
        "Введите текст уведомления (он будет отправлен клиентам):",
    )
    .reply_markup(KeyboardRemove::new())
    .await?;

    storage
        .set_state(msg.chat.id, State::SendNotification { customer, product })
        .await;

    Ok(())
}

async fn send_notification(
    msg: Message,
    params: ConfigParams,
    customer: String,
    product: String,
) -> HandlerResult {
    log::info!("add notification for user [{}]", msg.chat.id.0);
    let user_id = UserId::from(msg.chat.id.0);
    let timestamp = msg.date.timestamp();
    let notification = msg.text().unwrap().to_string();
    let message = CustomerRequest::NewNotification {
        user_id,
        customer,
        product,
        notification,
        timestamp,
    }
    .to_string();
    params
        .publisher
        .lock()
        .await
        .publish_message(&params.exchange, &params.request_queue, message)
        .await
        .unwrap();

    Ok(())
}

async fn callback_handler(
    q: CallbackQuery,
    bot: AutoSend<Bot>,
    params: ConfigParams,
) -> HandlerResult {
    if let Some(data) = q.data {
        log::info!("Callback [{}]", data);

        if let Some(message) = q.message {
            bot.delete_message(message.chat.id, message.id).await?;
            let command: Command = data.into();
            match command {
                Command::AddNotification => {
                    let user_id = UserId::from(message.chat.id.0);
                    let timestamp = message.date.timestamp();
                    let customer = params
                        .authorized_customers
                        .lock()
                        .await
                        .get(&message.chat.id)
                        .unwrap()
                        .name
                        .to_string();
                    let message = CustomerRequest::ProductsForNotification {
                        user_id,
                        customer,
                        timestamp,
                    }
                    .to_string();
                    params
                        .publisher
                        .lock()
                        .await
                        .publish_message(&params.exchange, &params.request_queue, message)
                        .await
                        .unwrap();
                }
            };
        }
    } else {
        log::info!("None of callback");
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Command {
    AddNotification,
}

impl From<Command> for String {
    fn from(command: Command) -> Self {
        serde_json::to_string(&command).unwrap()
    }
}
impl From<String> for Command {
    fn from(s: String) -> Self {
        serde_json::from_str(&s).unwrap()
    }
}
