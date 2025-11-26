use crate::app;
use crate::domain::ShutterPositions;
use log::info;

pub struct MockShuttersController {}

impl Default for MockShuttersController {
    fn default() -> Self {
        Self::new()
    }
}

impl MockShuttersController {
    pub fn new() -> Self {
        Self {}
    }
}

impl app::ShuttersController for MockShuttersController {
    fn set_shutter_positions(
        &self,
        shutter_positions: &ShutterPositions,
    ) -> crate::errors::Result<()> {
        info!("Setting shutter positions: {}", shutter_positions);
        Ok(())
    }
}
