pub struct Robot{
	pub id: u32,
	pub robot_behavior: RobotBehavior,
	pub robot_specialization: RobotSpecialization,
}

pub enum RobotBehavior {
	Explore,
	Collect,
    Communicate,
}

pub enum RobotSpecialization {
	Driller,
	ChemistryAnalyzer,
	Physicist,
}

pub trait RobotActions {
    fn return_to_station(&self);
}

impl RobotActions for Robot {
    fn return_to_station(&self) {
        println!("Robot {} is returning to the station", self.id);
    }
}

impl Robot {
    pub fn new(id: u32, robot_behavior: RobotBehavior, robot_specialization: RobotSpecialization) -> Self {
        Self {
            id,
            robot_behavior,
            robot_specialization,
        }
    }
}