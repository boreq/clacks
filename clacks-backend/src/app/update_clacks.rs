use crate::app;
use crate::app::{Clacks, ClacksUpdateResult, EventPublisher, Metrics};
use crate::errors::{Error, Result};
use clacks_macros::application_handler;

#[derive(Clone)]
pub struct UpdateClacksHandler<C, M, P> {
    clacks: C,
    metrics: M,
    publisher: P,
}

impl<C, M, P> UpdateClacksHandler<C, M, P> {
    pub fn new(clacks: C, metrics: M, publisher: P) -> Self {
        Self {
            clacks,
            metrics,
            publisher,
        }
    }
}

impl<C, M, P> app::UpdateClacksHandler for UpdateClacksHandler<C, M, P>
where
    C: Clacks,
    M: Metrics,
    P: EventPublisher,
{
    #[application_handler]
    fn handle(&self) -> Result<()> {
        match self.clacks.update()? {
            ClacksUpdateResult::StateChanged => {
                self.publisher.publish_clacks_updated()?;
            }
            ClacksUpdateResult::StateNotChanged => {}
        }
        Ok::<(), Error>(())
    }
}
