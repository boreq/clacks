use crate::app;
use crate::app::{AddMessageToQueue, Encoding, Metrics, Queue};
use crate::errors::Result;
use clacks_macros::application_handler;

pub struct AddMessageToQueueHandler<Q, M, E> {
    queue: Q,
    metrics: M,
    encoding: E,
}

impl<Q, M, E> AddMessageToQueueHandler<Q, M, E> {
    pub fn new(queue: Q, metrics: M, encoding: E) -> Self {
        Self {
            queue,
            metrics,
            encoding,
        }
    }
}

impl<Q, M, E> app::AddMessageToQueueHandler for AddMessageToQueueHandler<Q, M, E>
where
    Q: Queue,
    M: Metrics,
    E: Encoding,
{
    #[application_handler]
    fn handle(&self, add_message_to_queue: AddMessageToQueue) -> Result<()> {
        let encoded_message = self.encoding.encode(&add_message_to_queue.message)?;
        self.queue.add_message(encoded_message)
    }
}
