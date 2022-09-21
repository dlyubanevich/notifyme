use std::{sync::Arc, collections::HashMap};

use domain::{responses::Response, models::Customer};
use teloxide::{
    prelude::AutoSend,
    requests::Requester,
    types::{
        ChatId,
    },
    Bot,
};
use tokio::sync::Mutex;

use crate::{storage::StateStorage, customer::state::State, common::HandlerResult};

pub struct Service {
    bot: AutoSend<Bot>,
    state_storage: Arc<StateStorage<State>>,
    authorized_customers: Arc<Mutex<HashMap<ChatId, Customer>>>,
}

impl Service {
    pub fn new(bot: AutoSend<Bot>, state_storage: Arc<StateStorage<State>>, authorized_customers: Arc<Mutex<HashMap<ChatId, Customer>>>) -> Self {
        Service { bot, state_storage, authorized_customers }
    }
    pub async fn handle_response(&mut self, response: Response) -> HandlerResult {
        match response {
            Response::CustomerAuthorizationFailure { user_id } => {
                self.bot
                    .send_message(
                        ChatId(user_id as i64),
                        "Не верный ключ! Повторите попытку:",
                    )
                    .await?;
                    Ok(())
            },
            Response::CustomerAuthorizationSuccess { user_id, customer } => {
                let chat_id = ChatId(user_id as i64);
                let customer_name = customer.name.to_string();
                self.authorized_customers.lock().await.insert(chat_id, customer);
                self.state_storage.set_state(chat_id, State::Command).await;
                let text = format!("Добро пожаловать, {}", customer_name);
                self.bot
                    .send_message(
                        chat_id,
                        text,
                    )
                    .await?;
                Ok(())
            },
            Response::Subscriptions { user_id, products } => {
                let chat_id = ChatId(user_id as i64);
                let text = match products.len() {
                    0 => "Нет подписок на товары!".to_string(),
                    n => {
                        let list = products.join(";\n");
                        format!("У вас {n} подписки(ок) на следующие товары:\n{list}")
                    }
                }; 
                self.bot
                    .send_message(
                        chat_id,
                        text,
                    )
                    .await?; 
                self.state_storage.set_state(chat_id, State::Command).await;
                Ok(())
            }
            _ => todo!(),
        }
    }
}
