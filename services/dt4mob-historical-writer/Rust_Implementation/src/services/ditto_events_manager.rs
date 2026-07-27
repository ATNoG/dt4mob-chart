use tokio::sync::mpsc;

use crate::models::ditto_event::DittoEvent;

pub struct DittoEventsManager {
    sender: mpsc::Sender<DittoEvent>,
}

impl DittoEventsManager {
    pub fn new(sender: mpsc::Sender<DittoEvent>) -> Self {
        Self { sender }
    }

    pub async fn write(&self, event: DittoEvent) -> Result<(), mpsc::error::SendError<DittoEvent>> {
        self.sender.send(event).await
    }
}
