use std::{collections::HashMap, sync::Arc};

use domain::{models::Customer, responses::CustomerResponse};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::AutoSend,
    requests::Requester,
    types::{ChatId, KeyboardButton, KeyboardMarkup},
    Bot,
};
use tokio::sync::Mutex;

use crate::{common::HandlerResult, customer::state::State, storage::StateStorage};

pub struct CustomerService {
    bot: AutoSend<Bot>,
    state_storage: Arc<StateStorage<State>>,
    authorized_customers: Arc<Mutex<HashMap<ChatId, Customer>>>,
}

impl CustomerService {
    pub fn new(
        bot: AutoSend<Bot>,
        state_storage: Arc<StateStorage<State>>,
        authorized_customers: Arc<Mutex<HashMap<ChatId, Customer>>>,
    ) -> Self {
        CustomerService {
            bot,
            state_storage,
            authorized_customers,
        }
    }
    pub async fn handle_response(&mut self, response: CustomerResponse) -> HandlerResult {
        match response {
            CustomerResponse::AuthorizationFailure { user_id } => {
                let chat_id = ChatId(user_id.0 as i64);
                self.bot
                    .send_message(chat_id, "Указан не верный ключ!")
                    .await?;
                self.state_storage.set_state(chat_id, State::Start).await;
                Ok(())
            }
            CustomerResponse::AuthorizationSuccess { user_id, customer } => {
                let chat_id = ChatId(user_id.0 as i64);
                let customer_name = customer.name.to_string();
                self.authorized_customers
                    .lock()
                    .await
                    .insert(chat_id, customer);
                self.state_storage.set_state(chat_id, State::Start).await;
                let text = format!("Добро пожаловать, {}", customer_name);
                self.bot.send_message(chat_id, text).await?;
                Ok(())
            }
            CustomerResponse::ProductsForNotification {
                user_id,
                customer,
                products,
            } => {
                let chat_id = ChatId(user_id.0 as i64);
                match products.len() {
                    0 => {
                        self.bot
                            .send_message(chat_id, "Нет подписок на товары!")
                            .await?;
                        self.state_storage.set_state(chat_id, State::Start).await;
                    }
                    _ => {
                        let mut keyboard: Vec<Vec<KeyboardButton>> = vec![];
                        for products in products.chunks(3) {
                            let row = products
                                .iter()
                                .map(|product| KeyboardButton::new(product.name.clone()))
                                .collect();
                            keyboard.push(row);
                        }
                        self.bot
                            .send_message(chat_id, "Выберите товар:")
                            .reply_markup(KeyboardMarkup::new(keyboard))
                            .await?;
                        self.state_storage
                            .set_state(chat_id, State::AddNotification { customer })
                            .await;
                    }
                };

                Ok(())
            }
            CustomerResponse::NotificationSuccess { user_id } => {
                let chat_id = ChatId(user_id.0 as i64);
                self.bot
                    .send_message(chat_id, "Уведомление успешно отправлено!")
                    .await?;
                self.state_storage.set_state(chat_id, State::Start).await;
                Ok(())
            }
            CustomerResponse::NotificationFailure { user_id } => {
                let chat_id = ChatId(user_id.0 as i64);
                self.bot
                    .send_message(chat_id, "Не удалось отправить уведомление!")
                    .await?;
                self.state_storage.set_state(chat_id, State::Start).await;
                Ok(())
            }
            CustomerResponse::ClientSubscription {
                user_id, product, ..
            } => {
                let chat_id = ChatId(user_id.0 as i64);
                let text = format!("Оформлена подписка на товар [{product}]!");
                self.bot.send_message(chat_id, text).await?;
                Ok(())
            }
        }
    }
}
