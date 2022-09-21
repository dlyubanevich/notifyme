use std::{sync::Arc, collections:: HashMap};

use domain::{
    requests::Request, models::Customer,
};
use dotenv::dotenv;
use message_queue::{Client, Publisher};
use serde::{Deserialize, Serialize};
use telegram_bot::{customer::{response_delegate, MessageHandler, Service, state::State}, storage::StateStorage };
use teloxide::{
    dispatching::{update_listeners::webhooks, UpdateFilterExt},
    dptree,
    prelude::{AutoSend, Dispatcher, LoggingErrorHandler},
    requests::{Requester, RequesterExt},
    types::{CallbackQuery, Message, Update, ChatId, InlineKeyboardButton, InlineKeyboardMarkup},
    Bot, payloads::SendMessageSetters,
};

use tokio::sync::Mutex;
use url::Url;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_APP_LOG", "info");
    pretty_env_logger::init_custom_env("RUST_APP_LOG");

    log::info!("Starting purchase bot...");

    dotenv().ok();

    let token = std::env::var("CUSTOMER_TOKEN").unwrap();

    let bot = Bot::new(token).auto_send();
    let addr = ([127, 0, 0, 1], 8080).into();
    let url = Url::parse("https://0b74-95-27-196-32.eu.ngrok.io").unwrap();
    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
        .await
        .expect("Couldn't setup webhook");

    let state_storage = StateStorage::<State>::new();
    let authorized_customers = Arc::new(Mutex::new(HashMap::<ChatId, Customer>::new()));
    let service = Service::new(bot.clone(), state_storage.clone(), authorized_customers.clone());
    let queue_handler = Arc::new(Mutex::new(MessageHandler::new(service)));

    let mut client = Client::new("amqp://localhost:5672").await;
    client
        .with_consumer("response", response_delegate(queue_handler))
        .await;

    let message_handler = Update::filter_message().endpoint(message_handler);
    let callback_query_handler = Update::filter_callback_query().endpoint(callback_handler);

    let handler = dptree::entry()
        .branch(message_handler)
        .branch(callback_query_handler);

    
    let params = ConfigParams {
        authorized_customers,
        publisher: client.get_publisher(),
    };

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
        State::Start => start(bot, msg, storage, params).await?,
        State::Authorization => authorization(msg, params).await?,
        State::Command => choose_command(bot, msg).await?,
        State::Subscriptions => show_subscriptions(msg, params).await?,
        _ => todo!()
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
    if params.authorized_customers.lock().await.get(&msg.chat.id).is_some() {
        storage.set_state(msg.chat.id, State::Command).await; 
        choose_command(bot, msg).await?;   
    }else {
        bot.send_message(msg.chat.id, "Введите ключ для авторизации:").await?;
        storage.set_state(msg.chat.id, State::Authorization).await;
    }
    Ok(())
}

async fn authorization(
    msg: Message,
    params: ConfigParams,
) -> HandlerResult {
    log::info!("Authorization for user [{}]", msg.chat.id.0);
    let key = msg.text().unwrap().to_owned();
    let message = Request::CustomerAuthorization { user_id: msg.chat.id.0 as u32, key }.to_string();
    params
        .publisher
        .lock()
        .await
        .publish_message("notifyme", "request.customer", message)
        .await;
    Ok(())
}
async fn choose_command(
    bot: AutoSend<Bot>,
    msg: Message
) -> HandlerResult {
    log::info!("Choose command for user [{}]", msg.chat.id.0);
    let keyboard: Vec<Vec<InlineKeyboardButton>> = vec![
        vec![
            InlineKeyboardButton::callback("Показать подписки".to_owned(), Command::ShowSubscriptions), 
            InlineKeyboardButton::callback("Создать уведомление".to_owned(), Command::AddNotification)
        ]
    ];
    bot.send_message(msg.chat.id, "Что нужно сделать?").reply_markup(InlineKeyboardMarkup::new(keyboard)).await?; 

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Command {
    ShowSubscriptions,
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

async fn show_subscriptions(
    msg: Message,
    params: ConfigParams,
) -> HandlerResult {
    log::info!("show subscriptions for user [{}]", msg.chat.id.0);
    let user_id = msg.chat.id.0 as u32;
    let customer = params.authorized_customers.lock().await.get(&msg.chat.id).unwrap().to_owned();
    let message = Request::Subscriptions { user_id, customer }.to_string();
    params
        .publisher
        .lock()
        .await
        .publish_message("notifyme", "request.customer", message)
        .await;
    
    Ok(())
}
async fn callback_handler(q: CallbackQuery, bot: AutoSend<Bot>, storage: Arc<StateStorage<State>>,) -> HandlerResult {
    if let Some(data) = q.data {
        log::info!("Callback [{}]", data);
        
        if let Some(message) = q.message {
            bot.delete_message(message.chat.id, message.id).await?;
            let text = message.text().unwrap().to_owned();
            bot.send_message(message.chat.id, text).await?; 
            let command: Command = data.into();
            let state = match command {
                Command::ShowSubscriptions => State::Subscriptions,
                Command::AddNotification => State::Notification,
            };
            storage.set_state(message.chat.id, state).await;
        }

    } else {
        log::info!("None of callback");
    }

    Ok(())
}
