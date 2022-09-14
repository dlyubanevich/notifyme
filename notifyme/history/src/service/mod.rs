use domain::records::Record;

use crate::repository::SqliteRepository;


pub struct HistoryService {
    repository: SqliteRepository,
}

impl HistoryService {
    pub fn new(repository: SqliteRepository) -> Self {
        HistoryService { repository }
    }
    pub async fn add_record(&mut self, record: Record) {
        let result = match record {
            Record::UserEvent(record) => self.repository.add_user_event(record).await,
            Record::CustomerEvent(record) => self.repository.add_customer_event(record).await,
        };
    }
}