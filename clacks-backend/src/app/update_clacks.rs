use crate::app;
use crate::app::{Clacks, Metrics};
use crate::errors::Result;
use clacks_macros::application_handler;

#[derive(Clone)]
pub struct UpdateClacksHandler<C, M> {
    clacks: C,
    metrics: M,
}

impl<C, M> UpdateClacksHandler<C, M> {
    pub fn new(clacks: C, metrics: M) -> Self {
        Self { clacks, metrics }
    }
}

impl<C, M> app::UpdateClacksHandler for UpdateClacksHandler<C, M>
where
    C: Clacks,
    M: Metrics,
{
    #[application_handler]
    fn handle(&self) -> Result<()> {
        self.clacks.update()
    }
}
