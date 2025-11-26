use crate::domain::{ShutterLocation, ShutterLocationSide, ShutterPosition, ShutterPositions};
use crate::errors::Result;
use anyhow::anyhow;
use std::fmt::{Display, Formatter};

const MAX_SERVO_ID: u8 = 15;
pub const TRAVEL_RANGE: f32 = 90.0;

pub struct ShuttersController<SC> {
    servo_controller: SC,
}

impl<SC> ShuttersController<SC>
where
    SC: ServoController,
{
    pub fn new(servo_controller: SC) -> Self {
        Self { servo_controller }
    }

    pub fn set_shutter_positions(&self, shutter_positions: &ShutterPositions) -> Result<()> {
        for location in ShutterLocation::iter() {
            let position = shutter_positions.get_position(location);
            self.move_shutter(location, &position)?;
        }
        Ok(())
    }

    fn move_shutter(&self, shutter: &ShutterLocation, position: &ShutterPosition) -> Result<()> {
        let id = self.shutter_location_to_servo_id(shutter)?;

        let negative_angle = ServoAngle::new(-45.0)?;
        let positive_angle = ServoAngle::new(45.0)?;
        let angle = match shutter.side() {
            ShutterLocationSide::Left => match position {
                ShutterPosition::Open => &positive_angle,
                ShutterPosition::Closed => &negative_angle,
            },
            ShutterLocationSide::Right => match position {
                ShutterPosition::Open => &negative_angle,
                ShutterPosition::Closed => &positive_angle,
            },
        };

        self.servo_controller.rotate(&id, angle)
    }

    fn shutter_location_to_servo_id(&self, location: &ShutterLocation) -> Result<ServoID> {
        ServoID::new(match location {
            ShutterLocation::TopLeft => 0,
            ShutterLocation::TopRight => 1,
            ShutterLocation::MiddleLeft => 2,
            ShutterLocation::MiddleRight => 3,
            ShutterLocation::BottomLeft => 4,
            ShutterLocation::BottomRight => 5,
        })
    }
}

pub trait ServoController {
    fn rotate(&self, id: &ServoID, angle: &ServoAngle) -> Result<()>;
}

pub struct ServoID {
    id: u8,
}

impl ServoID {
    pub fn new(id: u8) -> Result<Self> {
        if id > MAX_SERVO_ID {
            return Err(anyhow!("ID must be lower or equal to {}", MAX_SERVO_ID).into());
        }
        Ok(ServoID { id })
    }

    pub fn id(&self) -> u8 {
        self.id
    }
}

impl Display for ServoID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<servo id: {}>", self.id)
    }
}

pub struct ServoAngle {
    angle: f32,
}

impl ServoAngle {
    fn new(angle: f32) -> Result<ServoAngle> {
        if !angle.is_finite() {
            return Err(anyhow!("angle must be finite").into());
        }
        let half_travel_range = TRAVEL_RANGE / 2.0;
        if angle.abs() > half_travel_range {
            return Err(anyhow!(
                "absolute value of angle must be lower or equal to {}",
                half_travel_range
            )
            .into());
        }
        Ok(ServoAngle { angle })
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }
}

impl Display for ServoAngle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<servo angle: {}>", self.angle)
    }
}
