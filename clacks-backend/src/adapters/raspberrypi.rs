use crate::domain::servos;
use crate::domain::servos::{ServoAngle, ServoID, TRAVEL_RANGE};
use crate::errors::Result;
use rppal::i2c::I2c;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

const MIN_PULSE_US: f32 = 1000.0;
const MAX_PULSE_US: f32 = 2000.0;
const PERIOD_US: f32 = 20_000.0;

pub struct ServoController {
    i2c: Arc<Mutex<I2c>>,
}

impl ServoController {
    pub fn new() -> Result<Self> {
        let mut i2c = I2c::new()?;
        i2c.set_slave_address(0x40)?;
        i2c.block_write(0x00, &[0x10])?;
        i2c.block_write(0xFE, &[0x79])?;
        i2c.block_write(0x00, &[0x20])?;

        // todo maybe remove but those things usually need a couple of ms to init
        thread::sleep(time::Duration::from_millis(100));

        Ok(Self {
            i2c: Arc::new(Mutex::new(i2c)),
        })
    }
}

impl servos::ServoController for ServoController {
    fn rotate(&self, id: &ServoID, angle: &ServoAngle) -> Result<()> {
        let i2c = self.i2c.lock().unwrap();
        let register = self.register(id.id());
        let command = self.servo_command(angle.angle());
        i2c.block_write(register, &command)?;
        Ok(())
    }
}

impl ServoController {
    fn register(&self, channel: u8) -> u8 {
        0x06 + channel * 4
    }

    fn servo_command(&self, angle: f32) -> [u8; 4] {
        let ticks = self.angle_to_ticks(angle);
        let off_l = (ticks & 0xFF) as u8;
        let off_h = (ticks >> 8) as u8;
        [0x00, 0x00, off_l, off_h]
    }

    fn angle_to_ticks(&self, angle: f32) -> u16 {
        let half_travel = TRAVEL_RANGE / 2.0;
        let pulse_us =
            MIN_PULSE_US + ((angle + half_travel) / TRAVEL_RANGE) * (MAX_PULSE_US - MIN_PULSE_US);
        ((pulse_us / PERIOD_US) * 4096.0).round() as u16
    }
}
