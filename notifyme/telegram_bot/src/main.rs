
use std::sync::Arc;

use domain::{models::{Customer, Product}, requests::Request};
use dotenv::dotenv;
use message_queue::{Client, Publisher};
use teloxide::{
    dispatching::{update_listeners::webhooks, dialogue::{InMemStorage, self}, UpdateFilterExt, HandlerExt},
    prelude::{AutoSend, Dispatcher, LoggingErrorHandler},
    requests::{Requester, RequesterExt},
    respond,
    types::{Message, Update},
    Bot, dptree,
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

    let client = Client::new("amqp://localhost:5672").await;
    let params = ConfigParams {
        publisher:  client.get_publisher()   
    };

    let bot = Bot::new(token).auto_send();
    let addr = ([127, 0, 0, 1], 8080).into();
    let url = Url::parse("https://5aff-95-27-196-48.eu.ngrok.io").unwrap();
    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
        .await
        .expect("Couldn't setup webhook");

    let message_handler = Update::filter_message()
        .enter_dialogue::<Update, InMemStorage<State>, State>()
        .branch(dptree::case![State::Start].endpoint(start))
        .branch(dptree::case![State::Customer].endpoint(set_customer))
        .branch(dptree::case![State::Product { customer }].endpoint(set_product))
        .branch(
            dptree::case![State::Confirm { customer, product }].endpoint(confirm),
        );

   // let callback_query_handler = Update::filter_callback_query().endpoint(callback);

    let handler = dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        //.branch(callback_query_handler)
        ;
   
     Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new(), params])
        .enable_ctrlc_handler()
        .build()
        .dispatch_with_listener(listener, LoggingErrorHandler::with_custom_text("main::An error from the update listener"))
        .await;
}

async fn answer(message: Message, bot: AutoSend<Bot>) -> Result<(), teloxide::RequestError> {
    log::info!("Message = [{}]", message.text().unwrap());
    bot.send_message(message.chat.id, "pong").await?;
    respond(())
}


#[derive(Clone)]
struct ConfigParams {
    publisher: Arc<Mutex<Publisher>>
}
type MyDialogue = teloxide::prelude::Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

async fn start(bot: AutoSend<Bot>, msg: Message, dialogue: MyDialogue, params: ConfigParams) -> HandlerResult { 
    log::info!("Start for user [{}]", msg.chat.id.0);
    let x = Request::Customers{user_id: msg.chat.id.0 as u32};
    let message = serde_json::to_string(&x).unwrap();
    params.publisher.lock().await.publish_message("notifyme", "request", message).await;
    bot.send_message(msg.chat.id, "Выберите поставщика:").await?;
    
    dialogue.update(State::Customer).await?;
    Ok(())
}

async fn set_customer(bot: AutoSend<Bot>, msg: Message, dialogue: MyDialogue, params: ConfigParams) -> HandlerResult {
    log::info!("Customer for user [{}]", msg.chat.id.0);
    match msg.text().map(|text| {
        let cus: Customer = serde_json::from_str(text).unwrap();
        cus
    }) {
        Some(customer) => {
            bot.send_message(msg.chat.id, "Выберите интересующий вас продукт:").await?;
            dialogue.update(State::Product { customer }).await?;    
        }
        None => {
            bot.send_message(msg.chat.id, "Выберите поставщика из представленного списка, пожалуйста!").await?;    
        },
    }
    Ok(())
}

async fn set_product(bot: AutoSend<Bot>, msg: Message, dialogue: MyDialogue, publisher: Arc<Mutex<Publisher>>, customer: Customer) -> HandlerResult {
    log::info!("Product for user [{}]", msg.chat.id.0);
    match msg.text().map(|text| {
        let product: Product = serde_json::from_str(text).unwrap();
        product
    }) {
        Some(product) => {
            bot.send_message(msg.chat.id, "Выберите интересующий вас продукт:").await?;
            dialogue.update(State::Confirm { customer, product }).await?;    
        }
        None => {
            bot.send_message(msg.chat.id, "Выберите продукт из представленного списка, пожалуйста!").await?;    
        },
    }
    Ok(())
}

async fn confirm(bot: AutoSend<Bot>, msg: Message, dialogue: MyDialogue, publisher: Arc<Mutex<Publisher>>, customer: Customer, product: Product) -> HandlerResult {
    log::info!("Confirm for user [{}]", msg.chat.id.0);
    match msg.text() {
        Some(product) => {
            bot.send_message(msg.chat.id, "Подписка успешно оформлена!").await?;
            dialogue.update(State::End).await?;    
        }
        None => {
            bot.send_message(msg.chat.id, "Выберите продукт из представленного списка, пожалуйста!").await?;    
        },
    }
    Ok(())
}


#[derive(Clone)]
enum State {
    Start,
    Customer,
    Product {customer: Customer},
    Confirm {customer: Customer, product: Product},
    End,
}
impl Default for State {
    fn default() -> Self {
        Self::Start
    }
}