use crate::app;
use crate::app::{AddMessageToQueue, Metrics, Queue};
use crate::errors::Result;
use clacks_macros::application_handler;

pub struct AddMessageToQueueHandler<Q, M> {
    queue: Q,
    metrics: M,
}

impl<Q, M> AddMessageToQueueHandler<Q, M> {
    pub fn new(queue: Q, metrics: M) -> Self {
        Self { queue, metrics }
    }
}

impl<Q, M> app::AddMessageToQueueHandler for AddMessageToQueueHandler<Q, M>
where
    Q: Queue,
    M: Metrics,
{
    #[application_handler]
    fn handle(&self, add_message_to_queue: AddMessageToQueue) -> Result<()> {
        self.queue.add_message(add_message_to_queue.message)
    }
}
