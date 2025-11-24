use crate::app;
use crate::app::{AddMessageToQueue, Encoding, EventPublisher, Metrics, Queue};
use crate::errors::{Error, Result};
use clacks_macros::application_handler;

#[derive(Clone)]
pub struct AddMessageToQueueHandler<Q, M, E, P> {
    queue: Q,
    metrics: M,
    encoding: E,
    publisher: P,
}

impl<Q, M, E, P> AddMessageToQueueHandler<Q, M, E, P> {
    pub fn new(queue: Q, metrics: M, encoding: E, publisher: P) -> Self {
        Self {
            queue,
            metrics,
            encoding,
            publisher,
        }
    }
}

impl<Q, M, E, P> app::AddMessageToQueueHandler for AddMessageToQueueHandler<Q, M, E, P>
where
    Q: Queue,
    M: Metrics,
    E: Encoding,
    P: EventPublisher,
{
    #[application_handler]
    fn handle(&self, add_message_to_queue: AddMessageToQueue) -> Result<()> {
        let encoded_message = self.encoding.encode(&add_message_to_queue.message)?;
        self.queue.add_message(encoded_message)?;
        self.publisher.publish_message_added_to_queue()?;
        Ok::<(), Error>(())
    }
}
