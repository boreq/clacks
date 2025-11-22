use crate::app::UpdateClacksHandler;
use log::{debug, error};
use std::time::Duration;
use tokio::time::sleep;

static UPDATE_CLACKS_EVERY: Duration = Duration::from_secs(5);

pub struct UpdateClacksTimer<H: UpdateClacksHandler> {
    handler: H,
}

impl<H> UpdateClacksTimer<H>
where
    H: UpdateClacksHandler,
{
    pub fn new(handler: H) -> Self {
        Self { handler }
    }

    pub async fn run(&mut self) {
        loop {
            match self.handler.handle() {
                Ok(_) => {
                    debug!("executed UpdateClacks in timer");
                }
                Err(err) => {
                    error!("error executing UpdateClacks in timer: {}", err);
                }
            }
            sleep(UPDATE_CLACKS_EVERY).await;
        }
    }
}
