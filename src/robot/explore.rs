use crate::robot::Robot;

pub struct Explore {
pub robot: Robot,
}

impl ExploreRobot {

	pub fn new(id: u32) -> Self {
		Self {
			robot: Robot::new(id, RobotBehavior::Explore),
		}
	}

	pub fn explore(&self) {
		println!("Robot {} is exploring", self.robot.id);
	}
}


