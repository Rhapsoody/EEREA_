use crate::robot::{Robot, RobotBehavior};

pub struct Communicate {
	pub robot: Robot,
}

impl Communicate {
	pub fn new(id: u32) -> Self {
			Self {
					robot: Robot::new(id, RobotBehavior::Communicate),
			}
	}

	pub fn communicate(&self) {
			println!("Robot {} is communicating", self.robot.id);
	}
}