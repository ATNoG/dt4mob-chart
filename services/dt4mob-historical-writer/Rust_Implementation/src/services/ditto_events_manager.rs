use tokio::sync::mpsc;

use crate::models::ditto_event::DittoEvent;

pub struct DittoEventsManager {
    sender: mpsc::Sender<DittoEvent>,
}

impl DittoEventsManager {
    pub fn new(sender: mpsc::Sender<DittoEvent>) -> Self {
        Self { sender }
    }

    pub fn try_write(&self, event: DittoEvent) -> Result<(), mpsc::error::TrySendError<DittoEvent>> {
        self.sender.try_send(event)
    }
}
