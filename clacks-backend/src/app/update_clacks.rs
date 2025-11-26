use crate::app;
use crate::app::{Clacks, ClacksUpdateResult, EventPublisher, Metrics, ShuttersController};
use crate::errors::{Error, Result};
use clacks_macros::application_handler;

#[derive(Clone)]
pub struct UpdateClacksHandler<C, M, P, SC> {
    clacks: C,
    metrics: M,
    publisher: P,
    shutters_controller: SC,
}

impl<C, M, P, SC> UpdateClacksHandler<C, M, P, SC> {
    pub fn new(clacks: C, metrics: M, publisher: P, shutters_controller: SC) -> Self {
        Self {
            clacks,
            metrics,
            publisher,
            shutters_controller,
        }
    }
}

impl<C, M, P, SC> app::UpdateClacksHandler for UpdateClacksHandler<C, M, P, SC>
where
    C: Clacks,
    M: Metrics,
    P: EventPublisher,
    SC: ShuttersController,
{
    #[application_handler]
    fn handle(&self) -> Result<()> {
        match self.clacks.update()? {
            ClacksUpdateResult::StateChanged => {
                let desired_shutter_positions = self.clacks.get_desired_shutter_positions();
                self.shutters_controller
                    .set_shutter_positions(&desired_shutter_positions)?;

                self.publisher.publish_clacks_updated()?;
            }
            ClacksUpdateResult::StateNotChanged => {}
        }
        Ok::<(), Error>(())
    }
}
