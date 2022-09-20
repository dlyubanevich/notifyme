use std::sync::Arc;

use domain::{
    requests::Request,
};
use dotenv::dotenv;
use message_queue::{Client, Publisher};
use telegram_bot::{response_delegate, MessageHandler, Service, StateStorage};
use teloxide::{
    dispatching::{update_listeners::webhooks, UpdateFilterExt},
    dptree,
    prelude::{AutoSend, Dispatcher, LoggingErrorHandler},
    requests::{Requester, RequesterExt},
    types::{CallbackQuery, Message, Update},
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

    let token = std::env::var("TELOXIDE_TOKEN").unwrap();

    let bot = Bot::new(token).auto_send();
    let addr = ([127, 0, 0, 1], 8080).into();
    let url = Url::parse("https://0b74-95-27-196-32.eu.ngrok.io").unwrap();
    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
        .await
        .expect("Couldn't setup webhook");

    let service = Service::new(bot.clone());
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

    let state_storage = StateStorage::<State>::new();
    let params = ConfigParams {
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
    let message = Request::Customers {
        user_id: msg.chat.id.0 as u32,
    }
    .to_string();
    params
        .publisher
        .lock()
        .await
        .publish_message("notifyme", "request", message)
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
    let message = Request::Products {
        user_id: msg.chat.id.0 as u32,
        customer: customer.clone(),
    }
    .to_string();
    params
        .publisher
        .lock()
        .await
        .publish_message("notifyme", "request", message)
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
    let message = Request::NewSubscription {
        user_id: msg.chat.id.0 as u32,
        customer,
        product,
    }
    .to_string();
    params
        .publisher
        .lock()
        .await
        .publish_message("notifyme", "request", message)
        .await;
    storage.set_state(msg.chat.id, State::End).await;
    Ok(())
}

async fn callback_handler(q: CallbackQuery, bot: AutoSend<Bot>) -> HandlerResult {
    if let Some(version) = q.data {
        log::info!("Callback [{}]", version);
        let text = format!("You chose: {version}");

        match q.message {
            Some(Message { id, chat, .. }) => {
                bot.edit_message_text(chat.id, id, text).await?;
            }
            None => {
                if let Some(id) = q.inline_message_id {
                    bot.edit_message_text_inline(id, text).await?;
                }
            }
        }

        log::info!("You chose: {}", version);
    } else {
        log::info!("None of callback");
    }

    Ok(())
}

#[derive(Clone)]
enum State {
    Start,
    Customer,
    Product { customer: String },
    End,
}
