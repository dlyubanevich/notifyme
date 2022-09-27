use domain::responses::ClientResponse;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::AutoSend,
    requests::Requester,
    types::{ChatId, KeyboardButton, KeyboardMarkup, KeyboardRemove},
    Bot,
};

use crate::common::HandlerResult;

pub struct ClientService {
    bot: AutoSend<Bot>,
}

impl ClientService {
    pub fn new(bot: AutoSend<Bot>) -> Self {
        ClientService { bot }
    }
    pub async fn handle_response(&mut self, response: ClientResponse) -> HandlerResult {
        match response {
            ClientResponse::Customers { user_id, customers } => {
                let mut keyboard: Vec<Vec<KeyboardButton>> = vec![];
                for customers in customers.chunks(3) {
                    let row = customers
                        .iter()
                        .map(|customer| KeyboardButton::new(customer.name.clone()))
                        .collect();
                    keyboard.push(row);
                }
                self.bot
                    .send_message(ChatId(user_id.0 as i64), "Выберите поставщика:")
                    .reply_markup(KeyboardMarkup::new(keyboard))
                    .await?;
                Ok(())
            }
            ClientResponse::Products { user_id, products } => {
                let mut keyboard: Vec<Vec<KeyboardButton>> = vec![];
                for products in products.chunks(3) {
                    let row = products
                        .iter()
                        .map(|product| KeyboardButton::new(product.name.clone()))
                        .collect();
                    keyboard.push(row);
                }
                self.bot
                    .send_message(
                        ChatId(user_id.0 as i64),
                        "Выберите интересующий вас продукт:",
                    )
                    .reply_markup(KeyboardMarkup::new(keyboard))
                    .await?;
                Ok(())
            }
            ClientResponse::SubscriptionSuccess { user_id } => {
                self.bot
                    .send_message(ChatId(user_id.0 as i64), "Подписка успешно оформлена!")
                    .reply_markup(KeyboardRemove::new())
                    .await?;
                Ok(())
            }
            ClientResponse::SubscriptionFailure { user_id } => {
                self.bot
                    .send_message(
                        ChatId(user_id.0 as i64),
                        "К сожалению, что-то пошло не так и подписка не оформлена!",
                    )
                    .reply_markup(KeyboardRemove::new())
                    .await?;
                Ok(())
            }
            ClientResponse::CustomerNotification {
                user_id,
                customer,
                product,
                notification,
            } => {
                let text = format!("Новое уведомление от поставщика [{customer}] для товара [{product}]: \n {notification}");
                self.bot
                    .send_message(ChatId(user_id.0 as i64), text)
                    .await?;
                Ok(())
            }
        }
    }
}
