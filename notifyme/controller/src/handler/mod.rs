
use domain::{
    requests::{ClientRequest, CustomerRequest},
    responses::{
        ClientResponseFromRepository, CustomerResponseFromRepository, ResponseFromRepository,
    },
};

use crate::ControllerService;

pub struct MessageHandler {
    service: ControllerService,
}

impl MessageHandler {
    pub fn new(service: ControllerService) -> Self {
        MessageHandler { service }
    }
    pub async fn handle_client_request(&mut self, message: String) {
        let request: ClientRequest = serde_json::from_str(&message).unwrap();
        self.service.handle_client_request(request).await;
    }
    pub async fn handle_customer_request(&mut self, message: String) {
        let request: CustomerRequest = serde_json::from_str(&message).unwrap();
        self.service.handle_customer_request(request).await;
    }
    pub async fn handle_client_response_from_repository(&mut self, message: String) {
        let response: ClientResponseFromRepository = serde_json::from_str(&message).unwrap();
        self.service
            .handle_client_response_from_repository(response)
            .await;
    }
    pub async fn handle_customer_response_from_repository(&mut self, message: String) {
        let response: CustomerResponseFromRepository = serde_json::from_str(&message).unwrap();
        self.service
            .handle_customer_response_from_repository(response)
            .await;
    }
    pub async fn handle_response_from_repository(&mut self, message: String) {
        let response: ResponseFromRepository = serde_json::from_str(&message).unwrap();
        self.service.handle_response_from_repository(response).await;
    }
}
