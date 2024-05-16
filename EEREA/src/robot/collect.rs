use crate::robot::{Robot, RobotBehavior};

pub struct Collect {
	pub robot: Robot,
}

impl Collect {
	pub fn new(id: u32) -> Self {
			Self {
					robot: Robot::new(id, RobotBehavior::Collect),
			}
	}

	pub fn collect(&self) {
			println!("Robot {} is collecting", self.robot.id);
	}
}
