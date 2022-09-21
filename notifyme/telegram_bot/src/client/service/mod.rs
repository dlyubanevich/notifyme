use domain::responses::Response;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::AutoSend,
    requests::Requester,
    types::{
        ChatId, InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup,
        KeyboardRemove,
    },
    Bot,
};

pub struct Service {
    bot: AutoSend<Bot>,
}

impl Service {
    pub fn new(bot: AutoSend<Bot>) -> Self {
        Service { bot }
    }
    pub async fn handle_response(&mut self, response: Response) {
        match response {
            Response::Customers { user_id, customers } => {
                let mut keyboard: Vec<Vec<KeyboardButton>> = vec![];
                for customers in customers.chunks(3) {
                    let row = customers
                        .iter()
                        .map(|customer| KeyboardButton::new(customer.clone()))
                        .collect();
                    keyboard.push(row);
                }
                self.bot
                    .send_message(ChatId(user_id as i64), "Выберите поставщика:")
                    .reply_markup(KeyboardMarkup::new(keyboard))
                    .await;
            }
            Response::Products { user_id, products } => {
                let mut keyboard: Vec<Vec<KeyboardButton>> = vec![];
                for products in products.chunks(3) {
                    let row = products
                        .iter()
                        .map(|product| KeyboardButton::new(product.clone()))
                        .collect();
                    keyboard.push(row);
                }
                self.bot
                    .send_message(ChatId(user_id as i64), "Выберите интересующий вас продукт:")
                    .reply_markup(KeyboardMarkup::new(keyboard))
                    .await;
            }
            Response::SubscriptionSuccess { user_id } => {
                self.bot
                    .send_message(ChatId(user_id as i64), "Подписка успешно оформлена!")
                    .reply_markup(KeyboardRemove::new())
                    .await;
            }
            Response::SubscriptionFailure { user_id } => {
                self.bot
                    .send_message(
                        ChatId(user_id as i64),
                        "К сожалению, что-то пошло не так и подписка не оформлена!",
                    )
                    .reply_markup(KeyboardRemove::new())
                    .await;
            }
            _ => todo!(),
        }
    }
}
