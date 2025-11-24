use crate::app;
use crate::app::{Clacks, Metrics, Queue, State};
use crate::errors::{Error, Result};
use clacks_macros::application_handler;

#[derive(Clone)]
pub struct GetStateHandler<C, Q, M> {
    clacks: C,
    queue: Q,
    metrics: M,
}

impl<C, Q, M> GetStateHandler<C, Q, M> {
    pub fn new(clacks: C, queue: Q, metrics: M) -> Self {
        Self {
            clacks,
            queue,
            metrics,
        }
    }
}

impl<C, Q, M> app::GetStateHandler for GetStateHandler<C, Q, M>
where
    C: Clacks,
    Q: Queue,
    M: Metrics,
{
    #[application_handler]
    fn get_state(&self) -> Result<State> {
        let current_message = self.clacks.current_message();
        let queue = self.queue.get_messages()?;
        Ok::<State, Error>(State {
            current_message,
            queue,
        })
    }
}
